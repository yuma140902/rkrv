use core::num::NonZeroUsize;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pte(usize);

#[allow(clippy::use_self)]
impl Pte {
    pub const V: usize = 1 << 0;
    pub const R: usize = 1 << 1;
    pub const W: usize = 1 << 2;
    pub const X: usize = 1 << 3;
    pub const U: usize = 1 << 4;
    pub const G: usize = 1 << 5;
    pub const A: usize = 1 << 6;
    pub const D: usize = 1 << 7;
    pub const RESERVED0_OFF: usize = 8;
    pub const RESERVED0_MASK: usize = 0b11 << Pte::RESERVED0_OFF;
    pub const PPN_OFF: usize = 10;
    pub const PPN_MASK: usize = 0xfff_ffff_ffff << Pte::PPN_OFF;
    pub const PPN0_OFF: usize = 10;
    pub const PPN0_MASK: usize = 0x1ff << Pte::PPN0_OFF;
    pub const PPN1_OFF: usize = 19;
    pub const PPN1_MASK: usize = 0x1ff << Pte::PPN1_OFF;
    pub const PPN2_OFF: usize = 28;
    pub const PPN2_MASK: usize = 0x3ff_ffff << Pte::PPN2_OFF;
    pub const RESERVED1_OFF: usize = 54;
    pub const RESERVED1_MASK: usize = 0x7f << Pte::RESERVED1_OFF;
    pub const PBMT_OFF: usize = 61;
    pub const PBMT_MASK: usize = 0b11 << Pte::PBMT_OFF;
    pub const N: usize = 1 << 63;

    pub const fn valid() -> Self {
        Self(Pte::V)
    }

    pub const fn invalid() -> Self {
        Self(0)
    }

    pub const fn is_valid(&self) -> bool {
        self.0 & Pte::V != 0
    }

    pub const fn set_valid(&mut self, valid: bool) -> &mut Self {
        if valid {
            self.0 |= Pte::V;
        } else {
            self.0 &= !Pte::V;
        }
        self
    }

    pub const fn is_readable(&self) -> bool {
        self.0 & Pte::R != 0
    }

    pub const fn set_readable(&mut self, readable: bool) -> &mut Self {
        if readable {
            self.0 |= Pte::R;
        } else {
            self.0 &= !Pte::R;
        }
        self
    }

    pub const fn is_writable(&self) -> bool {
        self.0 & Pte::W != 0
    }

    pub const fn set_writable(&mut self, writable: bool) -> &mut Self {
        if writable {
            self.0 |= Pte::W;
        } else {
            self.0 &= !Pte::W;
        }
        self
    }

    pub const fn is_executable(&self) -> bool {
        self.0 & Pte::X != 0
    }

    pub const fn set_executable(&mut self, executable: bool) -> &mut Self {
        if executable {
            self.0 |= Pte::X;
        } else {
            self.0 &= !Pte::X;
        }
        self
    }

    pub const fn is_user(&self) -> bool {
        self.0 & Pte::U != 0
    }

    pub const fn set_user(&mut self, user: bool) -> &mut Self {
        if user {
            self.0 |= Pte::U;
        } else {
            self.0 &= !Pte::U;
        }
        self
    }

    pub const fn is_global(&self) -> bool {
        self.0 & Pte::G != 0
    }

    pub const fn set_global(&mut self, global: bool) -> &mut Self {
        if global {
            self.0 |= Pte::G;
        } else {
            self.0 &= !Pte::G;
        }
        self
    }

    pub const fn is_accessed(&self) -> bool {
        self.0 & Pte::A != 0
    }

    pub const fn set_accessed(&mut self, accessed: bool) -> &mut Self {
        if accessed {
            self.0 |= Pte::A;
        } else {
            self.0 &= !Pte::A;
        }
        self
    }

    pub const fn is_dirty(&self) -> bool {
        self.0 & Pte::D != 0
    }

    pub const fn set_dirty(&mut self, dirty: bool) -> &mut Self {
        if dirty {
            self.0 |= Pte::D;
        } else {
            self.0 &= !Pte::D;
        }
        self
    }

    pub const fn ppn(&self) -> usize {
        (self.0 & Pte::PPN_MASK) >> Pte::PPN_OFF
    }

    pub const fn set_ppn(&mut self, ppn: usize) -> &mut Self {
        self.0 = (self.0 & !Pte::PPN_MASK) | ((ppn << Pte::PPN_OFF) & Pte::PPN_MASK);
        self
    }

    pub const fn ppn0(&self) -> usize {
        (self.0 & Pte::PPN0_MASK) >> Pte::PPN0_OFF
    }

    pub const fn set_ppn0(&mut self, ppn0: usize) -> &mut Self {
        self.0 = (self.0 & !Pte::PPN0_MASK) | ((ppn0 << Pte::PPN0_OFF) & Pte::PPN0_MASK);
        self
    }

    pub const fn ppn1(&self) -> usize {
        (self.0 & Pte::PPN1_MASK) >> Pte::PPN1_OFF
    }

    pub const fn set_ppn1(&mut self, ppn1: usize) -> &mut Self {
        self.0 = (self.0 & !Pte::PPN1_MASK) | ((ppn1 << Pte::PPN1_OFF) & Pte::PPN1_MASK);
        self
    }

    pub const fn ppn2(&self) -> usize {
        (self.0 & Pte::PPN2_MASK) >> Pte::PPN2_OFF
    }

    pub const fn set_ppn2(&mut self, ppn2: usize) -> &mut Self {
        self.0 = (self.0 & !Pte::PPN2_MASK) | ((ppn2 << Pte::PPN2_OFF) & Pte::PPN2_MASK);
        self
    }

    pub const fn is_leaf(&self) -> bool {
        self.is_readable() || self.is_writable() || self.is_executable()
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [Pte; 512],
}

impl PageTable {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            entries: [Pte(0); 512],
        }
    }
}

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pa(usize);

#[allow(clippy::use_self)]
impl Pa {
    pub const OFFSET: usize = 0xfff;
    pub const PPN_OFF: usize = 12;
    pub const PPN_MASK: usize = 0xfff_ffff_ffff << Pa::PPN_OFF;
    pub const PPN0_OFF: usize = 12;
    pub const PPN0_MASK: usize = 0x1ff << Pa::PPN0_OFF;
    pub const PPN1_OFF: usize = 21;
    pub const PPN1_MASK: usize = 0x1ff << Pa::PPN1_OFF;
    pub const PPN2_OFF: usize = 30;
    pub const PPN2_MASK: usize = 0x3ff_ffff << Pa::PPN2_OFF;

    pub const fn get(&self) -> usize {
        self.0
    }

    pub const fn offset(&self) -> usize {
        self.0 & Pa::OFFSET
    }

    pub const fn ppn(&self) -> usize {
        (self.0 & Pa::PPN_MASK) >> Pa::PPN_OFF
    }

    pub const fn ppn0(&self) -> usize {
        (self.0 & Pa::PPN0_MASK) >> Pa::PPN0_OFF
    }

    pub const fn ppn1(&self) -> usize {
        (self.0 & Pa::PPN1_MASK) >> Pa::PPN1_OFF
    }

    pub const fn ppn2(&self) -> usize {
        (self.0 & Pa::PPN2_MASK) >> Pa::PPN2_OFF
    }
}

impl core::ops::Add<usize> for Pa {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonNullPa(NonZeroUsize);

impl NonNullPa {
    pub const fn new(pa: usize) -> Option<Self> {
        match NonZeroUsize::new(pa) {
            Some(nonzero) => Some(Self(nonzero)),
            None => None,
        }
    }

    pub const fn pa(&self) -> Pa {
        Pa(self.0.get())
    }

    pub const fn get(&self) -> usize {
        self.0.get()
    }

    pub const fn get_nonzero(&self) -> NonZeroUsize {
        self.0
    }
}

impl core::ops::Add<usize> for NonNullPa {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.saturating_add(rhs))
    }
}

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Va(usize);

#[allow(clippy::use_self)]
impl Va {
    pub const OFFSET: usize = 0xfff;
    pub const VPN0_OFF: usize = 12;
    pub const VPN0_MASK: usize = 0x1ff << Va::VPN0_OFF;
    pub const VPN1_OFF: usize = 21;
    pub const VPN1_MASK: usize = 0x1ff << Va::VPN1_OFF;
    pub const VPN2_OFF: usize = 30;
    pub const VPN2_MASK: usize = 0x1ff << Va::VPN2_OFF;

    pub const fn offset(&self) -> usize {
        self.0 & Va::OFFSET
    }

    pub const fn vpn0(&self) -> usize {
        (self.0 & Va::VPN0_MASK) >> Va::VPN0_OFF
    }

    pub const fn vpn1(&self) -> usize {
        (self.0 & Va::VPN1_MASK) >> Va::VPN1_OFF
    }

    pub const fn vpn2(&self) -> usize {
        (self.0 & Va::VPN2_MASK) >> Va::VPN2_OFF
    }
}

pub const GIGAPAGE_SIZE: usize = 1024 * 1024 * 1024;
pub const MEGAPAGE_SIZE: usize = 2 * 1024 * 1024;
pub const PAGE_SIZE: usize = 4096;

static mut L2_ROOT_PT: PageTable = PageTable::new();
static mut L1_MMIO_PT: PageTable = PageTable::new();
static mut L0_UART0_PT: PageTable = PageTable::new();
static mut L1_RAM_PT: PageTable = PageTable::new();

unsafe fn make_identity_mapping() {
    // UART0: 0x1000_0000 - 0x1000_1000
    // RAM: 0x8000_0000 - 0x8800_0000

    let uart0_va = Va(0x1000_0000);
    let uart0_pa = Pa(0x1000_0000);
    let mmio_pt_pa = Pa(&raw mut L1_MMIO_PT as usize);
    let uart0_pt_pa = Pa(&raw mut L0_UART0_PT as usize);
    let ram_pt_pa = Pa(&raw mut L1_RAM_PT as usize);
    unsafe {
        assert!(uart0_pa.0.is_multiple_of(PAGE_SIZE));
        L2_ROOT_PT.entries[uart0_va.vpn2()] = *Pte::valid().set_ppn(mmio_pt_pa.ppn());
        L1_MMIO_PT.entries[uart0_va.vpn1()] = *Pte::valid().set_ppn(uart0_pt_pa.ppn());
        L0_UART0_PT.entries[uart0_va.vpn0()] = *Pte::valid()
            .set_readable(true)
            .set_writable(true)
            .set_accessed(true)
            .set_dirty(true)
            .set_ppn(uart0_pa.ppn());

        for ram_addr in (0x8000_0000..0x8800_0000).step_by(MEGAPAGE_SIZE) {
            let ram_va = Va(ram_addr);
            let ram_pa = Pa(ram_addr);
            assert!(ram_addr.is_multiple_of(MEGAPAGE_SIZE));

            if !L2_ROOT_PT.entries[ram_va.vpn2()].is_valid() {
                L2_ROOT_PT.entries[ram_va.vpn2()] = *Pte::valid().set_ppn(ram_pt_pa.ppn());
            }
            L1_RAM_PT.entries[ram_va.vpn1()] = *Pte::valid()
                .set_readable(true)
                .set_writable(true)
                .set_executable(true)
                .set_accessed(true)
                .set_dirty(true)
                .set_ppn(ram_pa.ppn());
        }
    }
}

/// # Safety
///
/// TODO
pub unsafe fn init() {
    unsafe {
        make_identity_mapping();
        const MODE_OFF: usize = 60;
        const MODE_SV39: usize = 8;
        let satp: usize = (MODE_SV39 << MODE_OFF) | (Pa(&raw mut L2_ROOT_PT as usize).ppn());
        core::arch::asm!(
            "csrw satp, {}",
            "sfence.vma zero, zero",
            in(reg) satp,
        )
    }
}
