#[macro_use]
extern crate error_chain;

pub mod edns;
pub mod error;
pub mod header;
pub mod header_flag;
pub mod message;
pub mod message_builder;
pub mod message_render;
pub mod name;
pub mod opcode;
pub mod question;
pub mod rcode;
pub mod rdata;
pub mod rdata_a;
pub mod rdata_aaaa;
pub mod rdata_cname;
pub mod rdata_dname;
pub mod rdata_mx;
pub mod rdata_naptr;
pub mod rdata_ns;
pub mod rdata_opt;
pub mod rdata_ptr;
pub mod rdata_soa;
pub mod rr_class;
pub mod rr_type;
pub mod rrset;
pub mod util;

pub use header::Header;
pub use header_flag::HeaderFlag;
pub use message::Message;
pub use message_builder::MessageBuilder;
pub use message_render::MessageRender;
pub use name::Name;
pub use name::NameRelation;
pub use opcode::Opcode;
pub use rcode::Rcode;
pub use rdata::RData;
pub use rdata_a::A;
pub use rdata_aaaa::AAAA;
pub use rdata_cname::CName;
pub use rdata_dname::DName;
pub use rdata_mx::MX;
pub use rdata_naptr::NAPTR;
pub use rdata_ns::NS;
pub use rdata_opt::OPT;
pub use rdata_ptr::PTR;
pub use rdata_soa::SOA;
pub use rr_class::RRClass;
pub use rr_type::RRType;
pub use rrset::RRTtl;
pub use rrset::RRset;