static INPUT: &str = "40541D900AEDC01A88002191FE2F45D1006A2FC2388D278D4653E3910020F2E2F3E24C007ECD7ABA6A200E6E8017F92C934CFA0E5290B569CE0F4BA5180213D963C00DC40010A87905A0900021B0D624C34600906725FFCF597491C6008C01B0004223342488A200F4378C9198401B87311A0C0803E600FC4887F14CC01C8AF16A2010021D1260DC7530042C012957193779F96AD9B36100907A00980021513E3943600043225C1A8EB2C3040043CC3B1802B400D3CA4B8D3292E37C30600B325A541D979606E384B524C06008E802515A638A73A226009CDA5D8026200D473851150401E8BF16E2ACDFB7DCD4F5C02897A5288D299D89CA6AA672AD5118804F592FC5BE8037000042217C64876000874728550D4C0149F29D00524ACCD2566795A0D880432BEAC79995C86483A6F3B9F6833397DEA03E401004F28CD894B9C48A34BC371CF7AA840155E002012E21260923DC4C248035299ECEB0AC4DFC0179B864865CF8802F9A005E264C25372ABAC8DEA706009F005C32B7FCF1BF91CADFF3C6FE4B3FB073005A6F93B633B12E0054A124BEE9C570004B245126F6E11E5C0199BDEDCE589275C10027E97BE7EF330F126DF3817354FFC82671BB5402510C803788DFA009CAFB14ECDFE57D8A766F0001A74F924AC99678864725F253FD134400F9B5D3004A46489A00A4BEAD8F7F1F7497C39A0020F357618C71648032BB004E4BBC4292EF1167274F1AA0078902262B0D4718229C8608A5226528F86008CFA6E802F275E2248C65F3610066274CEA9A86794E58AA5E5BDE73F34945E2008D27D2278EE30C489B3D20336D00C2F002DF480AC820287D8096F700288082C001DE1400C50035005AA2013E5400B10028C009600A74001EF2004F8400C92B172801F0F4C0139B8E19A8017D96A510A7E698800EAC9294A6E985783A400AE4A2945E9170";

struct BitStream {
    chars: Vec<char>,
    i: usize,
    buffer: u64,
    buffer_bits: u64,
    bits_read: u64,
}

impl BitStream {
    fn from(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            i: 0,
            buffer: 0,
            buffer_bits: 0,
            bits_read: 0,
        }
    }

    fn read_bits(&mut self, bits: u64) -> u64 {
        while self.buffer_bits < bits {
            self.buffer = self.buffer << 4 | self.chars[self.i].to_digit(16).unwrap() as u64;
            self.buffer_bits += 4;
            self.i += 1;
        }
        self.buffer_bits -= bits;
        let result = self.buffer >> (self.buffer_bits);
        self.buffer &= 2_u64.pow(self.buffer_bits as u32) - 1;
        self.bits_read += bits;
        result
    }
}

struct Packet {
    version: u64,
    type_id: u64,
    value: Option<u64>,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn from(stream: &mut BitStream) -> Self {
        let version = stream.read_bits(3);
        let type_id = stream.read_bits(3);
        let (value, sub_packets) = if type_id == 4 {
            let mut value = 0;
            loop {
                let is_last_group = stream.read_bits(1) == 0;
                value = value << 4 | stream.read_bits(4);
                if is_last_group {
                    break;
                }
            }
            (Some(value), Vec::new())
        } else {
            let length_type_id = stream.read_bits(1);
            let mut sub_packets = Vec::new();
            if length_type_id == 0 {
                let length_bits = stream.read_bits(15);
                let bits_read = stream.bits_read;
                while stream.bits_read - bits_read < length_bits {
                    sub_packets.push(Packet::from(stream));
                }
            } else {
                let length_packets = stream.read_bits(11);
                for _ in 0..length_packets {
                    sub_packets.push(Packet::from(stream));
                }
            }
            (None, sub_packets)
        };
        Packet {
            version,
            type_id,
            value,
            sub_packets,
        }
    }

    fn evaluate(&self) -> u64 {
        match self.type_id {
            0 => self.sub_packets.iter().map(Self::evaluate).sum(),
            1 => self.sub_packets.iter().map(Self::evaluate).product(),
            2 => self.sub_packets.iter().map(Self::evaluate).min().unwrap(),
            3 => self.sub_packets.iter().map(Self::evaluate).max().unwrap(),
            4 => self.value.unwrap(),
            5 => (self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate()) as u64,
            6 => (self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate()) as u64,
            7 => (self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate()) as u64,
            _ => unreachable!(),
        }
    }

    fn version_sum(&self) -> u64 {
        self.version + self.sub_packets.iter().map(Self::version_sum).sum::<u64>()
    }
}

fn main() {
    let mut stream = BitStream::from(INPUT);
    let packet = Packet::from(&mut stream);
    println!("Part one: {}", packet.version_sum());
    println!("Part two: {}", packet.evaluate());
}
