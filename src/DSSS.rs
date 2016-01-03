use std;
use util;

#[derive(Debug)]
pub enum ModType {
    BPSK,
    OQPSK,
}

#[derive(Debug)]
pub struct ChipMap {
    chips_per_symbol:   usize,
    bits_per_symbol:    usize,
    symbols:            Vec<u8>,
    chips:              Vec<u32>,
}

impl ChipMap {
    pub fn new(mod_type: ModType) -> ChipMap {
        match mod_type {
            ModType::BPSK  => ChipMap {chips_per_symbol: 15,
                                       bits_per_symbol:  1,
                                       symbols:          vec![0,1],
                                       chips:            vec![0x000009af, 0x00007650]},
            ModType::OQPSK => ChipMap {chips_per_symbol: 32,
                                       bits_per_symbol:  4,
                                       symbols:          vec![0x00, 0x01, 0x02, 0x03,
                                                              0x04, 0x05, 0x06, 0x07,
                                                              0x08, 0x09, 0x0a, 0x0b,
                                                              0x0c, 0x0d, 0x0e, 0x0f],
                                       chips:            vec![0x6077ae6c, 0x4e077ae6,
                                                              0x6ce077ae, 0x66ce077a,
                                                              0x2e6ce077, 0x7ae6ce07,
                                                              0x77ae6ce0, 0x077ae6ce,
                                                              0x1f885193, 0x31f88519,
                                                              0x131f8851, 0x1931f885,
                                                              0x51931f88, 0x051931f8,
                                                              0x0851931f, 0x78851931]},
        }
    }
}


#[derive(Debug)]
pub struct DSSS {
    pub threshold:  usize,
    map:            ChipMap,
}

impl DSSS {
    pub fn new(mod_type: ModType, threshold: usize) -> DSSS {
        let chip_map = ChipMap::new(mod_type);
        assert!(threshold <= chip_map.chips_per_symbol);
        DSSS {  threshold:  threshold,
                map:        chip_map,}
    }

    pub fn decode(&self, chips: u32) -> Option<u8> {
        let mut min_errors = std::usize::MAX;
        let mut symbol_match = std::u8::MAX;

        // TODO: is it safe to exit as soon as match is found less than the threshold?
        for i in 0..self.map.chips.len() {
            let errors = util::count_bit_errors(chips, self.map.chips[i]);
            if errors < min_errors {
                min_errors = errors;
                symbol_match = self.map.symbols[i];
            };
        }

        if min_errors <= self.threshold {
            Some(symbol_match)
        } else {
            None
        }
    }

    pub fn encode(&self, symbol: u8) -> u32 {
        self.map.chips[symbol as usize]
    }
}

#[test]
fn dsss_test () {
    let bpsk_dsss = DSSS::new(ModType::BPSK, 0);

    for symbol in bpsk_dsss.map.symbols.iter() {
        let encoded = bpsk_dsss.encode(*symbol);
        let decoded = bpsk_dsss.decode(encoded).unwrap();
        assert_eq!(*symbol, decoded);
    }
}
