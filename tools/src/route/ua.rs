
pub mod ua_api{
    use actix_web::{post, web};
    use laurel_actix::Data;
    use laurel_actix::types::{route, Autowired, RequestBody};
    use laurel_tool_api::ua::{UaBrowser, UaDevice, UaOs};

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/interface/tools/ua")
            .service(parse_ua)
        );
    }

    #[post("/parse")]
    pub async fn parse_ua(
        extractor: Autowired<ua_parser::Extractor<'_>>,
        body: RequestBody<laurel_tool_api::ua::UaParseReqBo>
    )
        -> route::Result<laurel_tool_api::ua::Ua>{
        let (ua_browser, ua_os, ua_device) = extractor.extract(body.ua.as_str());
        let browser = match ua_browser {
            Some(b) => Some(to_browser(b.into_owned())),
            _ => None,
        };
        let os = match ua_os {
            Some(o) => Some(to_os(o.into_owned())),
            _ => None,
        };
        let device = match ua_device {
            Some(d) => Some(to_device(d.into_owned())),
            _ => None,
        };
        Data!(
            laurel_tool_api::ua::Ua{
                browser,
                os,
                device,
            }
        )
    }


    fn to_device(value: ua_parser::device::Value) -> UaDevice{
        UaDevice{
            device: Some(value.device),
            brand: value.brand,
            model: value.model,
        }
    }

    fn to_os(value: ua_parser::os::Value) -> UaOs{
        UaOs{
            os: Some(value.os),
            major: value.major,
            minor: value.minor,
            patch: value.patch,
            patch_minor: value.patch_minor,
        }
    }

    fn to_browser(value: ua_parser::user_agent::Value) -> UaBrowser{
        UaBrowser{
            family: Some(value.family),
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}