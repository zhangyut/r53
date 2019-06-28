use error::Error;
use message_render::MessageRender;
use name::Name;
use util::{InputBuffer, OutputBuffer};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SRV {
    priority:u16,
    weight  :u16,
    port    :u16,
    target  :Name,
}
 
impl SRV {
    pub fn from_wire(buf: &mut InputBuffer, _len: u16) -> Result<Self, Error> {
        let priority = buf.read_u16()?;
        let weight = buf.read_u16()?;
        let port = buf.read_u16()?;
        let target = Name::from_wire(buf, false)?;
        Ok(SRV {
            priority: priority,
            weight: weight,
            port: port,
            target: target,
        })
    }

    pub fn rend(&self, render: &mut MessageRender) {
        render.write_u16(self.priority);
        render.write_u16(self.weight);
        render.write_u16(self.port);
        render.write_name(&self.target, true);
    }

    pub fn to_wire(&self, buf: &mut OutputBuffer) {
        buf.write_u16(self.priority);
        buf.write_u16(self.weight);
        buf.write_u16(self.port);
        self.target.to_wire(buf);
    }

    pub fn to_string(&self) -> String {
        [self.priority.to_string(), self.weight.to_string(), self.port.to_string(), self.target.to_string()].join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util::hex::from_hex;

    #[test]
    fn test_srv_to_wire() {
        //12 10 53 www.baidu.com.
        //000c000a003503777705626169647503636f6d00
        let raw = from_hex("000c000a00350377777705626169647503636f6d00").unwrap();
        let mut buf = InputBuffer::new(raw.as_slice());
        let srv = SRV::from_wire(&mut buf, raw.len() as u16).unwrap();
        assert_eq!(Ok(srv.priority), "12".parse());
        assert_eq!(Ok(srv.weight), "10".parse());
        assert_eq!(Ok(srv.port), "53".parse());
        assert_eq!(&srv.target, &Name::new("www.baidu.com", false).unwrap());

        let mut render = MessageRender::new();
        srv.rend(&mut render);
        assert_eq!(raw.as_slice(), render.data());
        assert_eq!(srv.to_string(), "12 10 53 www.baidu.com.");
    }
}
