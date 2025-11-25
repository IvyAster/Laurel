use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::dict_route::page_dict,
        crate::routes::dict_route::create_dict,
        crate::routes::dict_route::delete_dict,
        crate::routes::dict_route::update_dict,

        crate::routes::dict_route::page_dict_values,
        crate::routes::dict_route::create_dict_value,
        crate::routes::dict_route::delete_dict_value,
        crate::routes::dict_route::update_dict_value,
    ),
    components(
        schemas(laurel_common::types::PageQuery)
    ),
    tags(
        (name = "dictionaries", description = "字典管理接口")
    )
)]
pub struct DictApiDoc;
