use std::collections::HashMap;
use std::string::ToString;
use bon::Builder;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use laurel_common::{datetime_format, enum_options};
use laurel_common::types::{HappyEnum, IndexAble, PageQuery, SelectOption};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::schema::menu)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Menu{
    pub id: i64,
    pub app_id: String,
    pub menu_id: String,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_action_type: String,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub weight: i32,
    pub parent_id: String,
    pub authority: Option<String>,
    pub menu_status: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}


#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::schema::menu)]
#[derive(Debug, Serialize, Deserialize, Builder)]
//#[diesel(set_as_null)]
pub struct UpdatableMenu{
    pub menu_name: Option<String>,
    pub menu_action_type: Option<String>,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub weight: Option<i32>,
    pub parent_id: Option<String>,
    pub authority: Option<String>,
    pub menu_status: Option<String>,
    pub uts: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schema::menu)]
pub struct InsertAbleMenu<'a> {
    pub app_id: &'a str,
    pub menu_id: &'a str,
    pub menu_name: &'a str,
    pub menu_type: &'a str,
    pub menu_action_type: &'a str,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub weight: i32,
    pub parent_id: &'a str,
    pub authority: Option<String>,
    pub menu_status: &'a str,
    pub cts: &'a NaiveDateTime,
    pub uts: &'a NaiveDateTime,
}


#[derive(Debug, Builder, Default)]
pub struct MenuQuery<'a>{
    pub app_id: &'a str,
    pub menu_id: Option<&'a str>,
    pub menu_ids: Option<&'a Vec<String>>,
    pub menu_name: Option<&'a str>,
    pub menu_type: Option<&'a str>,
    pub menu_types: Option<&'a Vec<String>>,
    pub menu_action_type: Option<&'a str>,
    pub menu_route: Option<&'a str>,
    pub parent_id: Option<&'a str>,
    pub parent_ids: Option<&'a Vec<String>>,
    pub authority: Option<&'a str>,
    pub menu_status: Option<&'a str>,
    pub menu_statuses: Option<&'a Vec<String>>
}

impl<'a> From<&'a MenuQueryReq> for MenuQuery<'a>{
    fn from(value: &'a MenuQueryReq) -> Self {
        let mut query = MenuQuery::default();
        query.app_id = value.app_id.as_str();
        if let Some(param) = &value.menu_id && !param.is_empty(){
            query.menu_id = Some(param.as_str());
        }
        if let Some(param) = &value.menu_ids && !param.is_empty(){
            query.menu_ids = Some(param);
        }
        if let Some(param) = &value.menu_name && !param.is_empty(){
            query.menu_name = Some(param.as_str());
        }
        if let Some(param) = &value.menu_type && !param.is_empty(){
            query.menu_type = Some(param.as_str());
        }
        if let Some(param) = &value.menu_types && !param.is_empty(){
            query.menu_types = Some(param);
        }
        if let Some(param) = &value.menu_action_type && !param.is_empty(){
            query.menu_action_type = Some(param.as_str());
        }
        if let Some(param) = &value.menu_route && !param.is_empty(){
            query.menu_route = Some(param.as_str());
        }
        if let Some(param) = &value.parent_id && !param.is_empty(){
            query.menu_route = Some(param.as_str());
        }
        if let Some(param) = &value.parent_id && !param.is_empty(){
            query.parent_id = Some(param);
        }
        if let Some(param) = &value.parent_ids && !param.is_empty(){
            query.parent_ids = Some(param);
        }
        if let Some(param) = &value.authority && param.is_empty(){
            query.authority = Some(param.as_str());
        }
        if let Some(param) = &value.menu_status && !param.is_empty(){
            query.menu_status = Some(param.as_str());
        }
        if let Some(param) = &value.menu_statuses && !param.is_empty(){
            query.menu_statuses = Some(param);
        }
        query
    }
}

#[derive(Debug, Default, Builder, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MenuCreateReq{
    pub app_id: String,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_action_type: String,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub parent_id: Option<String>,
    pub authority: Option<String>,
    pub menu_status: Option<String>,
    pub weight: i32,
}

#[derive(Debug, Default, Builder, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MenuQueryReq{
    pub app_id: String,
    pub menu_id: Option<String>,
    pub menu_ids: Option<Vec<String>>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub menu_types: Option<Vec<String>>,
    pub menu_action_type: Option<String>,
    pub menu_route: Option<String>,
    pub parent_id: Option<String>,
    pub parent_ids: Option<Vec<String>>,
    pub authority: Option<String>,
    pub menu_status: Option<String>,
    pub menu_statuses: Option<Vec<String>>,
    #[serde(flatten)]
    pub pagination: Option<PageQuery>,
}

#[derive(Debug, Default, Builder, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MenuUpdateReq{
    pub menu_id: String,
    pub menu_name: Option<String>,
    pub menu_action_type: Option<String>,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub weight: Option<i32>,
    pub parent_id: Option<String>,
    pub authority: Option<String>,
    pub menu_status: Option<String>,
}


#[derive(Debug)]
pub enum MenuStatus {
    OPEN(&'static str, &'static str),
    CLOSED(&'static str, &'static str),
    DELETED(&'static str, &'static str),
}

static MENU_STATES: [MenuStatus; 3] = [
    MenuStatus::OPEN("open", "已开启"),
    MenuStatus::CLOSED("closed", "已关闭"),
    MenuStatus::DELETED("deleted", "已删除"),

];

impl HappyEnum<&'static str> for MenuStatus{
    fn take(&self) -> (&'static str, &'static str) {
        match self {
            MenuStatus::OPEN(x, y) | MenuStatus::CLOSED(x, y) | MenuStatus::DELETED(x, y) => (x, y),
        }
    }

    fn valid(key: &str) -> bool {
        MenuStatus::find_self(key).is_some()
    }

    fn find(key: &str) -> Option<&'static str> {
        MenuStatus::find_self(key).map(|t| t.take().1).or(None)
    }

    fn find_self(key: &str) -> Option<&'static MenuStatus>{
        for item in &MENU_STATES {
            if let Some(y) = match item {
                &MenuStatus::OPEN(x, _) => if x == key { Some(item) } else { None },
                &MenuStatus::CLOSED(x, _) => if x == key { Some(item) } else { None },
                &MenuStatus::DELETED(x, _) => if x == key { Some(item) } else { None },
            }{
                return Some(y)
            }
        }
        None
    }

    fn options() -> Vec<SelectOption<&'static str, &'static str>> {
        enum_options!(MENU_STATES)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum MenuType{
    MENU(&'static str, &'static str),
    BTN(&'static str, &'static str),
}


static MENU_ENUM_TYPES: [MenuType; 2] = [
    MenuType::MENU("menu", "菜单"),
    MenuType::BTN("btn", "按钮"),
];


impl HappyEnum<&'static str> for MenuType{
    fn take(&self) -> (&'static str, &'static str) {
        match self {
            MenuType::MENU(x, y) | MenuType::BTN(x, y) => (x, y),
        }
    }

    fn valid(key: &str) -> bool {
        MenuType::find_self(key).is_some()
    }


    fn find(key: &str) -> Option<&'static str> {
        MenuType::find_self(key).map(|t| t.take().1).or(None)
    }

    fn find_self(key: &str) -> Option<&'static Self> {
        for item in &MENU_ENUM_TYPES {
            if let Some(y) = match item {
                &MenuType::MENU(x, _) => if x == key { Some(item) } else { None },
                &MenuType::BTN(x, _) => if x == key { Some(item) } else { None },
            }{
                return Some(y)
            }
        }
        None
    }


    #[allow(deprecated)]
    fn options() -> Vec<SelectOption<&'static str, &'static str>>{
        enum_options!(MENU_ENUM_TYPES)
    }
}



pub enum MenuActionType{
    ROUTE(&'static str, &'static str),
    LINK(&'static str, &'static str),
    IFRAME(&'static str, &'static str),
}


static MENU_ACTIONS: [MenuActionType; 3] = [
    MenuActionType::ROUTE("route", "路由"),
    MenuActionType::LINK("link", "链接"),
    MenuActionType::IFRAME("iframe", "iframe"),
];

impl HappyEnum<&'static str> for MenuActionType{

    fn take(&self) -> (&'static str, &'static str){
        match self {
            MenuActionType::ROUTE(x, y) | MenuActionType::LINK(x, y) | MenuActionType::IFRAME(x, y) => (x, y),
        }
    }

    fn valid(key: &str) -> bool {
        MenuActionType::find_self(key).is_some()
    }


    fn find(key: &str) -> Option<&'static str> {
        MenuActionType::find_self(key).map(|t| t.take().1).or(None)
    }

    fn find_self(key: &str) -> Option<&'static Self> {
        for item in &MENU_ACTIONS {
            if let Some(y) = match item {
                &MenuActionType::ROUTE(x, _) => if x == key { Some(item) } else { None },
                &MenuActionType::LINK(x, _) => if x == key { Some(item) } else { None },
                &MenuActionType::IFRAME(x, _) => if x == key { Some(item) } else { None },
            }{
                return Some(y)
            }
        }
        None
    }


    fn options() -> Vec<SelectOption<&'static str, &'static str>>{
        enum_options!(MENU_ACTIONS)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct MenuVo{
    pub index: u32,
    pub app_id: String,
    pub menu_id: String,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_type_name: Option<&'static str>,
    pub menu_action_type: String,
    pub menu_action_type_name: Option<&'static str>,
    pub menu_icon: Option<String>,
    pub menu_route: Option<String>,
    pub route_param: Option<String>,
    pub weight: i32,
    pub parent_id: String,
    pub parent_name: Option<String>,
    pub authority: Option<String>,
    pub menu_status: String,
    pub menu_status_name: Option<&'static str>,
    pub cts: String,
    pub uts: String,
    pub children: Vec<MenuVo>,
}

impl IndexAble for MenuVo{
    fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }
}

impl From<&Menu>  for MenuVo{
    fn from(value: &Menu) -> Self {
        MenuVo{
            index: 0,
            app_id: value.app_id.clone(),
            menu_id: value.menu_id.clone(),
            menu_name: value.menu_name.clone(),
            menu_type: value.menu_type.clone(),
            menu_type_name: MenuType::find(&value.menu_type),
            menu_action_type: value.menu_action_type.clone(),
            menu_action_type_name: MenuActionType::find(&value.menu_action_type),
            menu_icon: value.menu_icon.clone(),
            menu_route: value.menu_route.clone(),
            route_param: value.route_param.clone(),
            weight: value.weight,
            parent_id: value.parent_id.clone(),
            parent_name: None,
            authority: value.authority.clone(),
            menu_status: value.menu_status.clone(),
            menu_status_name: MenuStatus::find(&value.menu_status),
            cts: datetime_format!(value.cts),
            uts: datetime_format!(value.uts),
            children: vec![],
        }
    }
}

impl From<Menu> for MenuVo{
    fn from(value: Menu) -> Self {
        MenuVo{
            index: 0,
            app_id: value.app_id,
            menu_id: value.menu_id,
            menu_name: value.menu_name,
            menu_type_name: MenuType::find(&value.menu_type),
            menu_type: value.menu_type,
            menu_action_type_name: MenuActionType::find(&value.menu_action_type),
            menu_action_type: value.menu_action_type,
            menu_icon: value.menu_icon,
            menu_route: value.menu_route,
            route_param: value.route_param.clone(),
            weight: value.weight,
            parent_id: value.parent_id,
            parent_name: None,
            authority: value.authority,
            menu_status_name: MenuStatus::find(&value.menu_status),
            menu_status: value.menu_status,
            cts: datetime_format!(value.cts),
            uts: datetime_format!(value.uts),
            children: vec![],
        }
    }
}

struct TempNode {
    id: String,
    pid: String,
    children_indices: Vec<usize>,
}

impl Menu{

    /// 零拷贝、纯迭代构建树（O(n) 时间，O(n) 空间）
    pub fn build_tree(menus: Vec<Menu>) -> Vec<MenuVo>{
        if menus.is_empty() {
            return Vec::new();
        }
        let len = menus.len();
        let mut temp_nodes: Vec<TempNode> = Vec::with_capacity(len);
        let mut id_2_index: HashMap<String, usize> = HashMap::with_capacity(len);

        // 1. 第一次遍历：转换 Node -> TempNode（空 children），建立 id->index 映射
        for (idx, node) in menus.iter().enumerate(){
            temp_nodes.push(TempNode{
                id: node.menu_id.clone(),
                pid: node.parent_id.clone(),
                children_indices: Vec::new(),
            });
            id_2_index.insert(temp_nodes[idx].id.clone(), idx);
        }

        // 2. 第二次遍历：填充父子关系 + 识别根节点
        let mut root_indices = Vec::new();
        for i in 0..len {
            let pid = &temp_nodes[i].pid;

            if pid == &temp_nodes[i].id {
                // pid == id 为根节点
                root_indices.push(i);
            } else if let Some(&parent_idx) = id_2_index.get(pid) {
                // 子节点：存储索引而非引用，避免借用冲突
                temp_nodes[parent_idx].children_indices.push(i);
            }
        }

        // 3. 计算每个节点的深度（BFS，确保自底向上处理顺序）
        let mut depth = vec![0; len];
        let mut queue = Vec::with_capacity(len);

        for &root_idx in &root_indices {
            queue.push(root_idx);
        }

        // 迭代式 BFS（避免递归）
        let mut i = 0;
        while i < queue.len() {
            let parent_idx = queue[i];
            // 克隆 children_indices 以避免借用冲突
            let children = temp_nodes[parent_idx].children_indices.clone();

            for &child_idx in &children {
                depth[child_idx] = depth[parent_idx] + 1;
                queue.push(child_idx);
            }
            i += 1;
        }

        // 4. 按深度降序排序（确保子节点先于父节点处理）
        let mut processing_order: Vec<usize> = (0..len).collect();
        processing_order.sort_unstable_by_key(|&idx| std::cmp::Reverse(depth[idx]));

        // 5. 第三次遍历：自底向上构建 NodeVo（零拷贝转移所有权）
        let mut built_vos: Vec<Option<MenuVo>> = vec![None; len];

        for idx in processing_order {
            let menu_id = &temp_nodes[idx].id;
            if let Some(menu_idx) = id_2_index.get(menu_id){
                let menu = &menus[menu_idx.clone()];
                let children: Vec<MenuVo> = temp_nodes[idx].children_indices
                    .iter()
                    .filter_map(|&child_idx| {
                        match built_vos[child_idx].take() {
                            Some(mut m) => {
                                m.parent_name = Some(menu.menu_name.clone());
                                Some(m)
                            },
                            _ => None,
                        }
                    })
                    .collect();
                //children.sort_by(|a, b| a.weight.cmp(&b.weight));
                let mut m = MenuVo::from(menu);
                m.children = children;
                if m.parent_id == m.menu_id{
                    m.parent_name = Some(m.menu_name.clone());
                }
                built_vos[idx] = Some(
                    m
                )
            }
        }
        // 6. 收集根节点（完成所有权转移）
        root_indices.into_iter()
            .filter_map(|idx| built_vos[idx].take())
            .collect()
    }

    pub fn build_menu_tree(menus: Vec<Menu>) -> Vec<MenuVo> {
        if menus.is_empty() {
            return Vec::new();
        }

        // 第一步：转换 Item 到 ItemVo 并建立索引
        let mut nodes: Vec<MenuVo> = Vec::with_capacity(menus.len());
        let mut id_to_index: HashMap<String, usize> = HashMap::new();
        let mut pid_to_children: HashMap<String, Vec<usize>> = HashMap::new();

        for (index, item) in menus.into_iter().enumerate() {
            id_to_index.insert(item.menu_id.clone(), index);
            nodes.push((&item).into());

            // 记录父子关系
            pid_to_children
                .entry(
                    //item.parent_id.clone()
                    if (&item.menu_id) == (&item.parent_id) { "never_leak_root".to_string() } else { item.parent_id.clone() }
                )
                .or_insert_with(Vec::new)
                .push(index);
        }

        // 第二步：构建树结构
        let mut result = Vec::new();
        let mut stack: Vec<usize> = Vec::new();
        let mut processed = vec![false; nodes.len()];

        // 先处理根节点（pid 为空的节点）
        if let Some(root_indices) = pid_to_children.get("never_leak_root") {
            for &root_index in root_indices {
                stack.push(root_index);
            }
        }

        // 使用栈进行迭代处理
        while let Some(current_index) = stack.pop() {
            if processed[current_index] {
                continue;
            }

            processed[current_index] = true;
            let current_id = nodes[current_index].menu_id.clone();

            // 处理当前节点的子节点
            if let Some(child_indices) = pid_to_children.get(&current_id) {
                for &child_index in child_indices {
                    if !processed[child_index] {
                        // 移动子节点数据
                        let child_node = std::mem::replace(
                            &mut nodes[child_index],
                            MenuVo::default()
                        );
                        nodes[current_index].children.push(child_node);
                        stack.push(child_index);
                    }
                }
            }

            // 如果是根节点，添加到最终结果
            if nodes[current_index].parent_id == nodes[current_index].menu_id {
                let root_node = std::mem::replace(
                    &mut nodes[current_index],
                    MenuVo::default()
                );
                result.push(root_node);
            }
        }

        result
    }
}

#[test]
fn test(){
    println!("{:?}", MenuType::options());
    println!("{:?}", MenuType::find(&"menu".to_string()));
    assert_eq!(true, MenuType::valid(&"menu".to_string()));
    assert_eq!(&MenuType::MENU("menu", "菜单"), MenuType::find_self(&"menu".to_string()).unwrap());
}