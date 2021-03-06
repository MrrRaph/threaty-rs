use crate::api::censys::models::{per_page::PerPage, virtual_hosts::VirtualHost};
use reqwest::RequestBuilder;
use std::collections::HashMap;
use std::net::IpAddr;

pub trait CensysAPI {
    fn search_hosts(
        self,
        query: Option<&str>,
        per_page: Option<PerPage<0, 100>>,
        virtual_hosts: Option<VirtualHost>,
        cursor: Option<&str>,
    ) -> RequestBuilder;

    fn aggregate_hosts(
        self,
        query: Option<&str>,
        field: &str,
        num_buckets: Option<i32>,
        virtual_hosts: Option<VirtualHost>,
    ) -> RequestBuilder;

    fn view_host(self, ip: IpAddr, at_time: Option<&str>) -> RequestBuilder;

    fn view_host_diff(
        self,
        ip: IpAddr,
        ip_b: Option<IpAddr>,
        at_time: Option<&str>,
        at_time_b: Option<&str>,
    ) -> RequestBuilder;

    fn view_host_events(
        self,
        ip: IpAddr,
        start_time: Option<&str>,
        end_time: Option<&str>,
        per_page: Option<PerPage<1, 50>>,
        cursor: Option<&str>,
        reversed: Option<bool>,
    ) -> RequestBuilder;

    fn view_host_names(
        self,
        ip: IpAddr,
        per_page: Option<PerPage<1, 1000>>,
        cursor: Option<&str>,
    ) -> RequestBuilder;

    fn get_comments_by_host(self, ip: IpAddr) -> RequestBuilder;
    fn add_comment_by_host(self, ip: IpAddr, contents: &str) -> RequestBuilder;
    fn get_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder;
    fn update_comment_by_host(self, ip: IpAddr, comment_id: &str, contents: &str)
        -> RequestBuilder;
    fn delete_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder;
    fn get_host_metadata(self) -> RequestBuilder;
    fn list_hosts_for_tag(self, id: &str) -> RequestBuilder;
    fn get_tags_by_host(self, ip: IpAddr) -> RequestBuilder;
    fn tag_host(self, ip: IpAddr, id: &str) -> RequestBuilder;
    fn untag_host(self, ip: IpAddr, id: &str) -> RequestBuilder;
    fn view_certificate(self, sha256: &str) -> RequestBuilder;

    fn search_certificates(
        self,
        query: &str,
        page: i32,
        fields: Vec<&str>,
        flatten: bool,
    ) -> RequestBuilder;

    fn generate_certificate_report(self, query: &str, field: &str, bucket: i32) -> RequestBuilder;
    fn bulk_certificate_lookup(self, fingerprints: Vec<&str>) -> RequestBuilder;
    fn get_hosts_by_cert(self, fingerprint: &str, cursor: Option<&str>) -> RequestBuilder;
    fn get_comments_by_cert(self, fingerprint: &str) -> RequestBuilder;
    fn add_comment_by_cert(self, fingerprint: &str, contents: &str) -> RequestBuilder;
    fn get_comment_by_cert(self, fingerprint: &str, comment_id: &str) -> RequestBuilder;

    fn update_comment_by_cert(
        self,
        fingerprint: &str,
        comment_id: &str,
        contents: &str,
    ) -> RequestBuilder;

    fn delete_comment_by_cert(self, fingerprint: &str, comment_id: &str) -> RequestBuilder;
    fn list_certificates_for_tag(self, id: &str) -> RequestBuilder;
    fn get_tags_by_cert(self, fingerprint: &str) -> RequestBuilder;
    fn tag_cert(self, fingerprint: &str, id: &str) -> RequestBuilder;
    fn untag_cert(self, fingerprint: &str, id: &str) -> RequestBuilder;
    fn get_series(self) -> RequestBuilder;
    fn view_series(self, series: &str) -> RequestBuilder;
    fn view_result(self, series: &str, result: &str) -> RequestBuilder;
    fn account(self) -> RequestBuilder;
    fn list_tags(self) -> RequestBuilder;
    fn create_tag(self, name: &str, metadata: HashMap<&str, &str>) -> RequestBuilder;
    fn get_tag(self, id: &str) -> RequestBuilder;
    fn update_tag(self, id: &str, name: &str, metadata: HashMap<&str, &str>) -> RequestBuilder;
    fn delete_tag(self, id: &str) -> RequestBuilder;
}
