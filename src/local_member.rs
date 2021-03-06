use std::marker::PhantomData;
use std::str::FromStr;
use std::fmt::Debug; 

pub trait LocalMemory {
    fn set_offset(&mut self, new_offsets: Vec<usize>);
    fn read(&self) -> String;
    fn write(&self, value: &str);
}

#[derive(Clone, Debug)]
pub struct LocalMember<T> {
    offsets:    Vec<usize>,
    _phantom:   PhantomData<*const T>,
}

impl<T> LocalMember<T> 
where T: Default + ToString + FromStr,
<T as FromStr>::Err: Debug
{
    pub fn new() -> LocalMember<T> {
        LocalMember {
            offsets:    Vec::new(),
            _phantom:   PhantomData
        }
    }
    pub fn get_offset(&self) -> usize {
        use std::ptr::copy_nonoverlapping;
        let mut offset = 0usize;
        for i in 0..self.offsets.len()-1 { 
            offset += self.offsets[i];
            unsafe {
                copy_nonoverlapping(offset as *const usize, &mut offset, 1);
            }
        } 
        offset += self.offsets[self.offsets.len()-1];
        offset
    }
}

impl LocalMemory for LocalMember<String> {
    fn set_offset(&mut self, new_offsets: Vec<usize>) {
        self.offsets = new_offsets;
    }
    fn read(&self) -> String {
        use std::ptr::copy_nonoverlapping;
        let offset = self.get_offset();
        let mut parts = Vec::<u8>::new(); 
        let mut addition_offset = 0usize;
        loop {
            let mut byte = 0u8;
            unsafe {
                copy_nonoverlapping((offset + addition_offset) as *const u8, &mut byte, 1);
            }
            if byte == 0 {
                break;
            }
            addition_offset += 1;
            parts.push(byte);
        }
        String::from_utf8(parts).unwrap()
    }
    fn write(&self, value: &str) {
        use std::ptr::copy_nonoverlapping;
        let offset = self.get_offset();
        let bytes = value.as_bytes();
        unsafe {
            copy_nonoverlapping(&bytes, offset as *mut &[u8], bytes.len());
            copy_nonoverlapping(&0u8, (offset + bytes.len()) as *mut u8, 1);
        }
    }
}

impl<T> LocalMemory for LocalMember<T> 
where T: Default + ToString + FromStr,
<T as FromStr>::Err: Debug
{
    default fn set_offset(&mut self, new_offsets: Vec<usize>) {
        self.offsets = new_offsets;
    }
    default fn read(&self) -> String {
        use std::ptr::copy_nonoverlapping;

        let offset = self.get_offset();
        let mut out : T = T::default();
        unsafe {
            copy_nonoverlapping(offset as *const T, &mut out, 1usize);
        }
        out.to_string()
    }
    default fn write(&self, value: &str) {
        use std::ptr::copy_nonoverlapping;

        let offset = self.get_offset();
        let out : T = value.parse().unwrap();
        unsafe {
            copy_nonoverlapping(&out, offset as *mut T, 1usize);
        }
    }
}
