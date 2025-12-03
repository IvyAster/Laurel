use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::dict::page_dict,
        crate::routes::dict::create_dict,
        crate::routes::dict::delete_dict,
        crate::routes::dict::update_dict,
        crate::routes::dict::page_dict_values,
        crate::routes::dict::create_dict_value,
        crate::routes::dict::delete_dict_value,
        crate::routes::dict::update_dict_value,
    ),
    components(
        schemas(laurel_common::types::PageQuery)
    ),
    tags(
        (name = "dictionaries", description = "字典管理接口")
    )
)]
pub struct SystemApiDoc;
