pub type Index = usize;

pub struct IdentityHasher {
    state: u64,
}

impl std::hash::Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state << 8 | u64::from(byte);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub struct BuildIdentityHasher;

impl std::hash::BuildHasher for BuildIdentityHasher {
    type Hasher = IdentityHasher;
    fn build_hasher(&self) -> IdentityHasher {
        IdentityHasher { state: 0 }
    }
}
