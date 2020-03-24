pub mod dhcp {
    pub mod constant;

    pub mod operations {
        pub mod fetch_dhcp_logs;
        pub mod filter_dhcp_logs;
        pub mod publish_dhcp_logs;
    }
}

pub mod ad {
    pub mod constant;

    pub mod operations {
        pub mod fetch_ad_logs;
        pub mod filter_ad_logs;
        pub mod publish_ad_logs;
    }
}

pub mod kafka {
    pub mod producer;
    pub mod constant;
}

pub mod utilities {
    pub mod store_text;
    pub mod constant;
    pub mod service_discovery;
    pub mod validate_service_discovery;
    pub mod configuration;
}
