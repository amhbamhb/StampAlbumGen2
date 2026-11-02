pub struct BootstrapHandlersState {
    pub selected: Option<(String, String)>,
    pub logout_requested: bool,
    pub reload_requested: bool,
}

pub struct ReviewHandlersState {
    pub reload_requested: bool,
    pub return_to_start_requested: bool,
}
