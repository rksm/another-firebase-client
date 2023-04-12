//! See https://developers.google.com/identity/protocols/oauth2/scopes
//!
//! Some scopes are re-used. Those are still listed in this file but are commented out.
//!
//! This file was created using
//!
//! ```ignore
//! use std::collections::HashSet;
//!
//! #[derive(Debug)]
//! struct Scopes {
//!     name: String,
//!     scopes: Vec<Scope>,
//! }
//!
//! impl Scopes {
//!     fn parse<S: AsRef<str>>(content: S) -> Vec<Self> {
//!         let mut scopes = Vec::new();
//!         let mut current_scope: Option<&mut Scopes> = None;
//!
//!         for line in content.as_ref().lines() {
//!             if line.starts_with("https:") {
//!                 let scope = current_scope.as_mut().expect("current_scope");
//!                 let (uri, comment) = line.split_once('\t').expect("uri/comment");
//!                 scope.scopes.push(Scope {
//!                     uri: uri.to_string(),
//!                     comment: comment.to_string(),
//!                 });
//!                 continue;
//!             }
//!
//!             scopes.push(Scopes {
//!                 name: line.to_string(),
//!                 scopes: Vec::new(),
//!             });
//!             current_scope = scopes.last_mut();
//!         }
//!
//!         scopes
//!     }
//!
//!     fn print(&self, seen: &mut HashSet<String>) {
//!         println!("// {}", self.name);
//!         for scope in &self.scopes {
//!             println!("// {}", scope.comment);
//!             let name = scope
//!                 .uri
//!                 .replace("https://www.googleapis.com/", "")
//!                 .replace("https://", "")
//!                 .replace('/', "_")
//!                 .replace('.', "_")
//!                 .replace('-', "_")
//!                 .to_uppercase();
//!             let was_printed_before = seen.contains(&scope.uri);
//!             seen.insert(scope.uri.clone());
//!             println!(
//!                 r#"{}pub const {}: &str = "{}";"#,
//!                 if was_printed_before { "// " } else { "" },
//!                 name,
//!                 scope.uri
//!             );
//!         }
//!     }
//! }
//!
//! #[derive(Debug)]
//! struct Scope {
//!     uri: String,
//!     comment: String,
//! }
//!
//! fn main() {
//!     let content = std::fs::read_to_string("scopes.txt").unwrap();
//!     let scopes = Scopes::parse(content);
//!
//!     let mut seen = HashSet::new();
//!     for scope in scopes {
//!         scope.print(&mut seen);
//!         println!();
//!     }
//! }
//! ```

// AI Platform Training & Prediction API, v1
// View and manage your data across Google Cloud Platform services
pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str =
    "https://www.googleapis.com/auth/cloud-platform.read-only";

// Access Approval API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Access Context Manager API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Ad Exchange Buyer API, v1.4
// Manage your Ad Exchange buyer account configuration
pub const AUTH_ADEXCHANGE_BUYER: &str = "https://www.googleapis.com/auth/adexchange.buyer";

// Ad Exchange Buyer API II, v2beta1
// Manage your Ad Exchange buyer account configuration
// pub const AUTH_ADEXCHANGE_BUYER: &str = "https://www.googleapis.com/auth/adexchange.buyer";

// AdMob API, v1
// See your AdMob data
pub const AUTH_ADMOB_READONLY: &str = "https://www.googleapis.com/auth/admob.readonly";
// See your AdMob data
pub const AUTH_ADMOB_REPORT: &str = "https://www.googleapis.com/auth/admob.report";

// AdSense Host API, v4.1
// View and manage your AdSense host data and associated accounts
pub const AUTH_ADSENSEHOST: &str = "https://www.googleapis.com/auth/adsensehost";

// AdSense Management API, v1.4
// View and manage your AdSense data
pub const AUTH_ADSENSE: &str = "https://www.googleapis.com/auth/adsense";
// View your AdSense data
pub const AUTH_ADSENSE_READONLY: &str = "https://www.googleapis.com/auth/adsense.readonly";

// Admin Data Transfer API, v1
// View and manage data transfers between users in your organization
pub const AUTH_ADMIN_DATATRANSFER: &str = "https://www.googleapis.com/auth/admin.datatransfer";
// View data transfers between users in your organization
pub const AUTH_ADMIN_DATATRANSFER_READONLY: &str =
    "https://www.googleapis.com/auth/admin.datatransfer.readonly";

// Admin Directory API, v1
// View and manage customer related information
pub const AUTH_ADMIN_DIRECTORY_CUSTOMER: &str =
    "https://www.googleapis.com/auth/admin.directory.customer";
// View customer related information
pub const AUTH_ADMIN_DIRECTORY_CUSTOMER_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.customer.readonly";
// View and manage your Chrome OS devices' metadata
pub const AUTH_ADMIN_DIRECTORY_DEVICE_CHROMEOS: &str =
    "https://www.googleapis.com/auth/admin.directory.device.chromeos";
// View your Chrome OS devices' metadata
pub const AUTH_ADMIN_DIRECTORY_DEVICE_CHROMEOS_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly";
// View and manage your mobile devices' metadata
pub const AUTH_ADMIN_DIRECTORY_DEVICE_MOBILE: &str =
    "https://www.googleapis.com/auth/admin.directory.device.mobile";
// Manage your mobile devices by performing administrative tasks
pub const AUTH_ADMIN_DIRECTORY_DEVICE_MOBILE_ACTION: &str =
    "https://www.googleapis.com/auth/admin.directory.device.mobile.action";
// View your mobile devices' metadata
pub const AUTH_ADMIN_DIRECTORY_DEVICE_MOBILE_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly";
// View and manage the provisioning of domains for your customers
pub const AUTH_ADMIN_DIRECTORY_DOMAIN: &str =
    "https://www.googleapis.com/auth/admin.directory.domain";
// View domains related to your customers
pub const AUTH_ADMIN_DIRECTORY_DOMAIN_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.domain.readonly";
// View and manage the provisioning of groups on your domain
pub const AUTH_ADMIN_DIRECTORY_GROUP: &str =
    "https://www.googleapis.com/auth/admin.directory.group";
// View and manage group subscriptions on your domain
pub const AUTH_ADMIN_DIRECTORY_GROUP_MEMBER: &str =
    "https://www.googleapis.com/auth/admin.directory.group.member";
// View group subscriptions on your domain
pub const AUTH_ADMIN_DIRECTORY_GROUP_MEMBER_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.group.member.readonly";
// View groups on your domain
pub const AUTH_ADMIN_DIRECTORY_GROUP_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.group.readonly";
// View and manage organization units on your domain
pub const AUTH_ADMIN_DIRECTORY_ORGUNIT: &str =
    "https://www.googleapis.com/auth/admin.directory.orgunit";
// View organization units on your domain
pub const AUTH_ADMIN_DIRECTORY_ORGUNIT_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.orgunit.readonly";
// View and manage the provisioning of calendar resources on your domain
pub const AUTH_ADMIN_DIRECTORY_RESOURCE_CALENDAR: &str =
    "https://www.googleapis.com/auth/admin.directory.resource.calendar";
// View calendar resources on your domain
pub const AUTH_ADMIN_DIRECTORY_RESOURCE_CALENDAR_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly";
// Manage delegated admin roles for your domain
pub const AUTH_ADMIN_DIRECTORY_ROLEMANAGEMENT: &str =
    "https://www.googleapis.com/auth/admin.directory.rolemanagement";
// View delegated admin roles for your domain
pub const AUTH_ADMIN_DIRECTORY_ROLEMANAGEMENT_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly";
// View and manage the provisioning of users on your domain
pub const AUTH_ADMIN_DIRECTORY_USER: &str = "https://www.googleapis.com/auth/admin.directory.user";
// View and manage user aliases on your domain
pub const AUTH_ADMIN_DIRECTORY_USER_ALIAS: &str =
    "https://www.googleapis.com/auth/admin.directory.user.alias";
// View user aliases on your domain
pub const AUTH_ADMIN_DIRECTORY_USER_ALIAS_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.user.alias.readonly";
// View users on your domain
pub const AUTH_ADMIN_DIRECTORY_USER_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.user.readonly";
// Manage data access permissions for users on your domain
pub const AUTH_ADMIN_DIRECTORY_USER_SECURITY: &str =
    "https://www.googleapis.com/auth/admin.directory.user.security";
// View and manage the provisioning of user schemas on your domain
pub const AUTH_ADMIN_DIRECTORY_USERSCHEMA: &str =
    "https://www.googleapis.com/auth/admin.directory.userschema";
// View user schemas on your domain
pub const AUTH_ADMIN_DIRECTORY_USERSCHEMA_READONLY: &str =
    "https://www.googleapis.com/auth/admin.directory.userschema.readonly";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Admin Reports API, v1
// View audit reports for your G Suite domain
pub const AUTH_ADMIN_REPORTS_AUDIT_READONLY: &str =
    "https://www.googleapis.com/auth/admin.reports.audit.readonly";
// View usage reports for your G Suite domain
pub const AUTH_ADMIN_REPORTS_USAGE_READONLY: &str =
    "https://www.googleapis.com/auth/admin.reports.usage.readonly";

// Analytics Reporting API, v4
// View and manage your Google Analytics data
pub const AUTH_ANALYTICS: &str = "https://www.googleapis.com/auth/analytics";
// See and download your Google Analytics data
pub const AUTH_ANALYTICS_READONLY: &str = "https://www.googleapis.com/auth/analytics.readonly";

// Android Management API, v1
// Manage Android devices and apps for your customers
pub const AUTH_ANDROIDMANAGEMENT: &str = "https://www.googleapis.com/auth/androidmanagement";

// Apigee API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// App Engine Admin API, v1
// View and manage your applications deployed on Google App Engine
pub const AUTH_APPENGINE_ADMIN: &str = "https://www.googleapis.com/auth/appengine.admin";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";

// Apps Script API, v1
// Read, compose, send, and permanently delete all your email from Gmail
pub const MAIL_GOOGLE_COM_: &str = "https://mail.google.com/";
// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
pub const WWW_GOOGLE_COM_CALENDAR_FEEDS: &str = "https://www.google.com/calendar/feeds";
// See, edit, download, and permanently delete your contacts
pub const WWW_GOOGLE_COM_M8_FEEDS: &str = "https://www.google.com/m8/feeds";
// View and manage the provisioning of groups on your domain
// pub const AUTH_ADMIN_DIRECTORY_GROUP: &str = "https://www.googleapis.com/auth/admin.directory.group";
// View and manage the provisioning of users on your domain
// pub const AUTH_ADMIN_DIRECTORY_USER: &str = "https://www.googleapis.com/auth/admin.directory.user";
// View and manage your Google Docs documents
pub const AUTH_DOCUMENTS: &str = "https://www.googleapis.com/auth/documents";
// See, edit, create, and delete all of your Google Drive files
pub const AUTH_DRIVE: &str = "https://www.googleapis.com/auth/drive";
// View and manage your forms in Google Drive
pub const AUTH_FORMS: &str = "https://www.googleapis.com/auth/forms";
// View and manage forms that this application has been installed in
pub const AUTH_FORMS_CURRENTONLY: &str = "https://www.googleapis.com/auth/forms.currentonly";
// View and manage your Google Groups
pub const AUTH_GROUPS: &str = "https://www.googleapis.com/auth/groups";
// Create and update Google Apps Script deployments
pub const AUTH_SCRIPT_DEPLOYMENTS: &str = "https://www.googleapis.com/auth/script.deployments";
// View Google Apps Script deployments
pub const AUTH_SCRIPT_DEPLOYMENTS_READONLY: &str =
    "https://www.googleapis.com/auth/script.deployments.readonly";
// View Google Apps Script project's metrics
pub const AUTH_SCRIPT_METRICS: &str = "https://www.googleapis.com/auth/script.metrics";
// View Google Apps Script processes
pub const AUTH_SCRIPT_PROCESSES: &str = "https://www.googleapis.com/auth/script.processes";
// Create and update Google Apps Script projects
pub const AUTH_SCRIPT_PROJECTS: &str = "https://www.googleapis.com/auth/script.projects";
// View Google Apps Script projects
pub const AUTH_SCRIPT_PROJECTS_READONLY: &str =
    "https://www.googleapis.com/auth/script.projects.readonly";
// See, edit, create, and delete your spreadsheets in Google Drive
pub const AUTH_SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
// View your email address
pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";

// BigQuery API, v2
// View and manage your data in Google BigQuery
pub const AUTH_BIGQUERY: &str = "https://www.googleapis.com/auth/bigquery";
// Insert data into Google BigQuery
pub const AUTH_BIGQUERY_INSERTDATA: &str = "https://www.googleapis.com/auth/bigquery.insertdata";
// View your data in Google BigQuery
pub const AUTH_BIGQUERY_READONLY: &str = "https://www.googleapis.com/auth/bigquery.readonly";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// Manage your data and permissions in Google Cloud Storage
pub const AUTH_DEVSTORAGE_FULL_CONTROL: &str =
    "https://www.googleapis.com/auth/devstorage.full_control";
// View your data in Google Cloud Storage
pub const AUTH_DEVSTORAGE_READ_ONLY: &str = "https://www.googleapis.com/auth/devstorage.read_only";
// Manage your data in Google Cloud Storage
pub const AUTH_DEVSTORAGE_READ_WRITE: &str =
    "https://www.googleapis.com/auth/devstorage.read_write";

// BigQuery Connection API, v1beta1
// View and manage your data in Google BigQuery
// pub const AUTH_BIGQUERY: &str = "https://www.googleapis.com/auth/bigquery";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// BigQuery Data Transfer API, v1
// View and manage your data in Google BigQuery
// pub const AUTH_BIGQUERY: &str = "https://www.googleapis.com/auth/bigquery";
// View your data in Google BigQuery
// pub const AUTH_BIGQUERY_READONLY: &str = "https://www.googleapis.com/auth/bigquery.readonly";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";

// BigQuery Reservation API, v1
// View and manage your data in Google BigQuery
// pub const AUTH_BIGQUERY: &str = "https://www.googleapis.com/auth/bigquery";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Binary Authorization API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Blogger API, v3
// Manage your Blogger account
pub const AUTH_BLOGGER: &str = "https://www.googleapis.com/auth/blogger";
// View your Blogger account
pub const AUTH_BLOGGER_READONLY: &str = "https://www.googleapis.com/auth/blogger.readonly";

// Books API, v1
// Manage your books
pub const AUTH_BOOKS: &str = "https://www.googleapis.com/auth/books";

// Calendar API, v3
// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
pub const AUTH_CALENDAR: &str = "https://www.googleapis.com/auth/calendar";
// View and edit events on all your calendars
pub const AUTH_CALENDAR_EVENTS: &str = "https://www.googleapis.com/auth/calendar.events";
// View events on all your calendars
pub const AUTH_CALENDAR_EVENTS_READONLY: &str =
    "https://www.googleapis.com/auth/calendar.events.readonly";
// See and download any calendar you can access using your Google Calendar
pub const AUTH_CALENDAR_READONLY: &str = "https://www.googleapis.com/auth/calendar.readonly";
// View your Calendar settings
pub const AUTH_CALENDAR_SETTINGS_READONLY: &str =
    "https://www.googleapis.com/auth/calendar.settings.readonly";

// Campaign Manager 360 API, v3.4
// Manage DoubleClick Digital Marketing conversions
pub const AUTH_DDMCONVERSIONS: &str = "https://www.googleapis.com/auth/ddmconversions";
// View and manage DoubleClick for Advertisers reports
pub const AUTH_DFAREPORTING: &str = "https://www.googleapis.com/auth/dfareporting";
// View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns
pub const AUTH_DFATRAFFICKING: &str = "https://www.googleapis.com/auth/dfatrafficking";

// Chrome Verified Access API, v1
// Verify your enterprise credentials
pub const AUTH_VERIFIEDACCESS: &str = "https://www.googleapis.com/auth/verifiedaccess";

// Cloud Asset API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Bigtable Admin API, v2
// Administer your Cloud Bigtable tables and clusters
pub const AUTH_BIGTABLE_ADMIN: &str = "https://www.googleapis.com/auth/bigtable.admin";
// Administer your Cloud Bigtable clusters
pub const AUTH_BIGTABLE_ADMIN_CLUSTER: &str =
    "https://www.googleapis.com/auth/bigtable.admin.cluster";
// Administer your Cloud Bigtable clusters
pub const AUTH_BIGTABLE_ADMIN_INSTANCE: &str =
    "https://www.googleapis.com/auth/bigtable.admin.instance";
// Administer your Cloud Bigtable tables
pub const AUTH_BIGTABLE_ADMIN_TABLE: &str = "https://www.googleapis.com/auth/bigtable.admin.table";
// Administer your Cloud Bigtable tables and clusters
pub const AUTH_CLOUD_BIGTABLE_ADMIN: &str = "https://www.googleapis.com/auth/cloud-bigtable.admin";
// Administer your Cloud Bigtable clusters
pub const AUTH_CLOUD_BIGTABLE_ADMIN_CLUSTER: &str =
    "https://www.googleapis.com/auth/cloud-bigtable.admin.cluster";
// Administer your Cloud Bigtable tables
pub const AUTH_CLOUD_BIGTABLE_ADMIN_TABLE: &str =
    "https://www.googleapis.com/auth/cloud-bigtable.admin.table";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";

// Cloud Billing API, v1
// View and manage your Google Cloud Platform billing accounts
pub const AUTH_CLOUD_BILLING: &str = "https://www.googleapis.com/auth/cloud-billing";
// View your Google Cloud Platform billing accounts
pub const AUTH_CLOUD_BILLING_READONLY: &str =
    "https://www.googleapis.com/auth/cloud-billing.readonly";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Build API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Composer API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud DNS API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// View your DNS records hosted by Google Cloud DNS
pub const AUTH_NDEV_CLOUDDNS_READONLY: &str =
    "https://www.googleapis.com/auth/ndev.clouddns.readonly";
// View and manage your DNS records hosted by Google Cloud DNS
pub const AUTH_NDEV_CLOUDDNS_READWRITE: &str =
    "https://www.googleapis.com/auth/ndev.clouddns.readwrite";

// Cloud Data Loss Prevention (DLP) API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Dataproc API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Datastore API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your Google Cloud Datastore data
pub const AUTH_DATASTORE: &str = "https://www.googleapis.com/auth/datastore";

// Cloud Debugger API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Use Stackdriver Debugger
pub const AUTH_CLOUD_DEBUGGER: &str = "https://www.googleapis.com/auth/cloud_debugger";

// Cloud Deployment Manager V2 API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// View and manage your Google Cloud Platform management resources and deployment status information
pub const AUTH_NDEV_CLOUDMAN: &str = "https://www.googleapis.com/auth/ndev.cloudman";
// View your Google Cloud Platform management resources and deployment status information
pub const AUTH_NDEV_CLOUDMAN_READONLY: &str =
    "https://www.googleapis.com/auth/ndev.cloudman.readonly";

// Cloud Filestore API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Firestore API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your Google Cloud Datastore data
// pub const AUTH_DATASTORE: &str = "https://www.googleapis.com/auth/datastore";

// Cloud Functions API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Healthcare API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Identity API, v1
// See your device details
pub const AUTH_CLOUD_IDENTITY_DEVICES_LOOKUP: &str =
    "https://www.googleapis.com/auth/cloud-identity.devices.lookup";
// See, change, create, and delete any of the Cloud Identity Groups that you can access, including the members of each group
pub const AUTH_CLOUD_IDENTITY_GROUPS: &str =
    "https://www.googleapis.com/auth/cloud-identity.groups";
// See any Cloud Identity Groups that you can access, including group members and their emails
pub const AUTH_CLOUD_IDENTITY_GROUPS_READONLY: &str =
    "https://www.googleapis.com/auth/cloud-identity.groups.readonly";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Identity-Aware Proxy API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud IoT API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Register and manage devices in the Google Cloud IoT service
pub const AUTH_CLOUDIOT: &str = "https://www.googleapis.com/auth/cloudiot";

// Cloud Key Management Service (KMS) API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your keys and secrets stored in Cloud Key Management Service
pub const AUTH_CLOUDKMS: &str = "https://www.googleapis.com/auth/cloudkms";

// Cloud Life Sciences API, v2beta
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Logging API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// Administrate log data for your projects
pub const AUTH_LOGGING_ADMIN: &str = "https://www.googleapis.com/auth/logging.admin";
// View log data for your projects
pub const AUTH_LOGGING_READ: &str = "https://www.googleapis.com/auth/logging.read";
// Submit log data for your projects
pub const AUTH_LOGGING_WRITE: &str = "https://www.googleapis.com/auth/logging.write";

// Cloud Memorystore for Memcached API, v1beta2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Monitoring API, v3
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and write monitoring data for all of your Google and third-party Cloud and API projects
pub const AUTH_MONITORING: &str = "https://www.googleapis.com/auth/monitoring";
// View monitoring data for all of your Google Cloud and third-party projects
pub const AUTH_MONITORING_READ: &str = "https://www.googleapis.com/auth/monitoring.read";
// Publish metric data to your Google Cloud projects
pub const AUTH_MONITORING_WRITE: &str = "https://www.googleapis.com/auth/monitoring.write";

// Cloud Natural Language API, v1
// Apply machine learning models to reveal the structure and meaning of text
pub const AUTH_CLOUD_LANGUAGE: &str = "https://www.googleapis.com/auth/cloud-language";
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud OS Login API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your Google Compute Engine resources
pub const AUTH_COMPUTE: &str = "https://www.googleapis.com/auth/compute";

// Cloud Pub/Sub API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage Pub/Sub topics and subscriptions
pub const AUTH_PUBSUB: &str = "https://www.googleapis.com/auth/pubsub";

// Cloud Resource Manager API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";

// Cloud Run Admin API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Runtime Configuration API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Manage your Google Cloud Platform services' runtime configuration
pub const AUTH_CLOUDRUNTIMECONFIG: &str = "https://www.googleapis.com/auth/cloudruntimeconfig";

// Cloud SQL Admin API, v1beta4
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Manage your Google SQL Service instances
pub const AUTH_SQLSERVICE_ADMIN: &str = "https://www.googleapis.com/auth/sqlservice.admin";

// Cloud Scheduler API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Search API, v1
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH: &str = "https://www.googleapis.com/auth/cloud_search";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_DEBUG: &str = "https://www.googleapis.com/auth/cloud_search.debug";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_INDEXING: &str =
    "https://www.googleapis.com/auth/cloud_search.indexing";
// Search your organization's data in the Cloud Search index
pub const AUTH_CLOUD_SEARCH_QUERY: &str = "https://www.googleapis.com/auth/cloud_search.query";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_SETTINGS: &str =
    "https://www.googleapis.com/auth/cloud_search.settings";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_SETTINGS_INDEXING: &str =
    "https://www.googleapis.com/auth/cloud_search.settings.indexing";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_SETTINGS_QUERY: &str =
    "https://www.googleapis.com/auth/cloud_search.settings.query";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_STATS: &str = "https://www.googleapis.com/auth/cloud_search.stats";
// Index and serve your organization's data with Cloud Search
pub const AUTH_CLOUD_SEARCH_STATS_INDEXING: &str =
    "https://www.googleapis.com/auth/cloud_search.stats.indexing";

// Cloud Shell API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Source Repositories API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Manage your source code repositories
pub const AUTH_SOURCE_FULL_CONTROL: &str = "https://www.googleapis.com/auth/source.full_control";
// View the contents of your source code repositories
pub const AUTH_SOURCE_READ_ONLY: &str = "https://www.googleapis.com/auth/source.read_only";
// Manage the contents of your source code repositories
pub const AUTH_SOURCE_READ_WRITE: &str = "https://www.googleapis.com/auth/source.read_write";

// Cloud Spanner API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Administer your Spanner databases
pub const AUTH_SPANNER_ADMIN: &str = "https://www.googleapis.com/auth/spanner.admin";
// View and manage the contents of your Spanner databases
pub const AUTH_SPANNER_DATA: &str = "https://www.googleapis.com/auth/spanner.data";

// Cloud Speech-to-Text API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Storage JSON API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// Manage your data and permissions in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_FULL_CONTROL: &str = "https://www.googleapis.com/auth/devstorage.full_control";
// View your data in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_READ_ONLY: &str = "https://www.googleapis.com/auth/devstorage.read_only";
// Manage your data in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_READ_WRITE: &str = "https://www.googleapis.com/auth/devstorage.read_write";

// Cloud TPU API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Tasks API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Testing API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";

// Cloud Text-to-Speech API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Tool Results API, v1beta3
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Trace API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Write Trace data for a project or application
pub const AUTH_TRACE_APPEND: &str = "https://www.googleapis.com/auth/trace.append";

// Cloud Translation API, v3
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Translate text from one language to another using Google Translate
pub const AUTH_CLOUD_TRANSLATION: &str = "https://www.googleapis.com/auth/cloud-translation";

// Cloud Video Intelligence API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Cloud Vision API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Apply machine learning models to understand and label images
pub const AUTH_CLOUD_VISION: &str = "https://www.googleapis.com/auth/cloud-vision";

// Compute Engine API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your Google Compute Engine resources
// pub const AUTH_COMPUTE: &str = "https://www.googleapis.com/auth/compute";
// View your Google Compute Engine resources
pub const AUTH_COMPUTE_READONLY: &str = "https://www.googleapis.com/auth/compute.readonly";
// Manage your data and permissions in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_FULL_CONTROL: &str = "https://www.googleapis.com/auth/devstorage.full_control";
// View your data in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_READ_ONLY: &str = "https://www.googleapis.com/auth/devstorage.read_only";
// Manage your data in Google Cloud Storage
// pub const AUTH_DEVSTORAGE_READ_WRITE: &str = "https://www.googleapis.com/auth/devstorage.read_write";

// Container Analysis API, v1beta1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Content API for Shopping, v2.1
// Manage your product listings and accounts for Google Shopping
pub const AUTH_CONTENT: &str = "https://www.googleapis.com/auth/content";

// Dataflow API, v1b3
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage your Google Compute Engine resources
// pub const AUTH_COMPUTE: &str = "https://www.googleapis.com/auth/compute";
// View your Google Compute Engine resources
// pub const AUTH_COMPUTE_READONLY: &str = "https://www.googleapis.com/auth/compute.readonly";
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";

// Display & Video 360 API, v1
// Create, see, edit, and permanently delete your Display & Video 360 entities and reports
pub const AUTH_DISPLAY_VIDEO: &str = "https://www.googleapis.com/auth/display-video";
// Create, see, and edit Display & Video 360 Campaign entities and see billing invoices
pub const AUTH_DISPLAY_VIDEO_MEDIAPLANNING: &str =
    "https://www.googleapis.com/auth/display-video-mediaplanning";
//
pub const AUTH_DISPLAY_VIDEO_USER_MANAGEMENT: &str =
    "https://www.googleapis.com/auth/display-video-user-management";
// View and manage your reports in DoubleClick Bid Manager
pub const AUTH_DOUBLECLICKBIDMANAGER: &str =
    "https://www.googleapis.com/auth/doubleclickbidmanager";

// DoubleClick Bid Manager API, v1.1
// View and manage your reports in DoubleClick Bid Manager
// pub const AUTH_DOUBLECLICKBIDMANAGER: &str = "https://www.googleapis.com/auth/doubleclickbidmanager";

// Drive API, v3
// See, edit, create, and delete all of your Google Drive files
// pub const AUTH_DRIVE: &str = "https://www.googleapis.com/auth/drive";
// View and manage its own configuration data in your Google Drive
pub const AUTH_DRIVE_APPDATA: &str = "https://www.googleapis.com/auth/drive.appdata";
// View and manage Google Drive files and folders that you have opened or created with this app
pub const AUTH_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
// View and manage metadata of files in your Google Drive
pub const AUTH_DRIVE_METADATA: &str = "https://www.googleapis.com/auth/drive.metadata";
// View metadata for files in your Google Drive
pub const AUTH_DRIVE_METADATA_READONLY: &str =
    "https://www.googleapis.com/auth/drive.metadata.readonly";
// View the photos, videos and albums in your Google Photos
pub const AUTH_DRIVE_PHOTOS_READONLY: &str =
    "https://www.googleapis.com/auth/drive.photos.readonly";
// See and download all your Google Drive files
pub const AUTH_DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
// Modify your Google Apps Script scripts' behavior
pub const AUTH_DRIVE_SCRIPTS: &str = "https://www.googleapis.com/auth/drive.scripts";

// Drive Activity API, v2
// View and add to the activity record of files in your Google Drive
pub const AUTH_DRIVE_ACTIVITY: &str = "https://www.googleapis.com/auth/drive.activity";
// View the activity record of files in your Google Drive
pub const AUTH_DRIVE_ACTIVITY_READONLY: &str =
    "https://www.googleapis.com/auth/drive.activity.readonly";

// Enterprise License Manager API, v1
// View and manage G Suite licenses for your domain
pub const AUTH_APPS_LICENSING: &str = "https://www.googleapis.com/auth/apps.licensing";

// Error Reporting API, v1beta1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Fact Check Tools API, v1alpha1
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";

// Firebase Cloud Messaging API, v1
// See, edit, configure, and delete your Google Cloud Platform data
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Firebase Dynamic Links API, v1
// View and administer all your Firebase data and settings
pub const AUTH_FIREBASE: &str = "https://www.googleapis.com/auth/firebase";

// Firebase Management API, v1beta1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// View and administer all your Firebase data and settings
// pub const AUTH_FIREBASE: &str = "https://www.googleapis.com/auth/firebase";
// View all your Firebase data and settings
pub const AUTH_FIREBASE_READONLY: &str = "https://www.googleapis.com/auth/firebase.readonly";

// Firebase Rules API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and administer all your Firebase data and settings
// pub const AUTH_FIREBASE: &str = "https://www.googleapis.com/auth/firebase";
// View all your Firebase data and settings
// pub const AUTH_FIREBASE_READONLY: &str = "https://www.googleapis.com/auth/firebase.readonly";

// Fitness API, v1
// Use Google Fit to see and store your physical activity data
pub const AUTH_FITNESS_ACTIVITY_READ: &str =
    "https://www.googleapis.com/auth/fitness.activity.read";
// See and add to your Google Fit physical activity data
pub const AUTH_FITNESS_ACTIVITY_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.activity.write";
// See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app.
pub const AUTH_FITNESS_BLOOD_GLUCOSE_READ: &str =
    "https://www.googleapis.com/auth/fitness.blood_glucose.read";
// See and add info about your blood glucose to Google Fit. I consent to Google sharing my blood glucose information with this app.
pub const AUTH_FITNESS_BLOOD_GLUCOSE_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.blood_glucose.write";
// See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
pub const AUTH_FITNESS_BLOOD_PRESSURE_READ: &str =
    "https://www.googleapis.com/auth/fitness.blood_pressure.read";
// See and add info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
pub const AUTH_FITNESS_BLOOD_PRESSURE_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.blood_pressure.write";
// See info about your body measurements and heart rate in Google Fit
pub const AUTH_FITNESS_BODY_READ: &str = "https://www.googleapis.com/auth/fitness.body.read";
// See and add info about your body measurements and heart rate to Google Fit
pub const AUTH_FITNESS_BODY_WRITE: &str = "https://www.googleapis.com/auth/fitness.body.write";
// See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
pub const AUTH_FITNESS_BODY_TEMPERATURE_READ: &str =
    "https://www.googleapis.com/auth/fitness.body_temperature.read";
// See and add to info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
pub const AUTH_FITNESS_BODY_TEMPERATURE_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.body_temperature.write";
// See your heart rate data in Google Fit. I consent to Google sharing my heart rate information with this app.
pub const AUTH_FITNESS_HEART_RATE_READ: &str =
    "https://www.googleapis.com/auth/fitness.heart_rate.read";
// See and add to your heart rate data in Google Fit. I consent to Google sharing my heart rate information with this app.
pub const AUTH_FITNESS_HEART_RATE_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.heart_rate.write";
// See your Google Fit speed and distance data
pub const AUTH_FITNESS_LOCATION_READ: &str =
    "https://www.googleapis.com/auth/fitness.location.read";
// See and add to your Google Fit location data
pub const AUTH_FITNESS_LOCATION_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.location.write";
// See info about your nutrition in Google Fit
pub const AUTH_FITNESS_NUTRITION_READ: &str =
    "https://www.googleapis.com/auth/fitness.nutrition.read";
// See and add to info about your nutrition in Google Fit
pub const AUTH_FITNESS_NUTRITION_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.nutrition.write";
// See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
pub const AUTH_FITNESS_OXYGEN_SATURATION_READ: &str =
    "https://www.googleapis.com/auth/fitness.oxygen_saturation.read";
// See and add info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
pub const AUTH_FITNESS_OXYGEN_SATURATION_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.oxygen_saturation.write";
// See info about your reproductive health in Google Fit. I consent to Google sharing my reproductive health information with this app.
pub const AUTH_FITNESS_REPRODUCTIVE_HEALTH_READ: &str =
    "https://www.googleapis.com/auth/fitness.reproductive_health.read";
// See and add info about your reproductive health in Google Fit. I consent to Google sharing my reproductive health information with this app.
pub const AUTH_FITNESS_REPRODUCTIVE_HEALTH_WRITE: &str =
    "https://www.googleapis.com/auth/fitness.reproductive_health.write";
// See your sleep data in Google Fit. I consent to Google sharing my sleep information with this app.
pub const AUTH_FITNESS_SLEEP_READ: &str = "https://www.googleapis.com/auth/fitness.sleep.read";
// See and add to your sleep data in Google Fit. I consent to Google sharing my sleep information with this app.
pub const AUTH_FITNESS_SLEEP_WRITE: &str = "https://www.googleapis.com/auth/fitness.sleep.write";

// G Suite Vault API, v1
// Manage your eDiscovery data
pub const AUTH_EDISCOVERY: &str = "https://www.googleapis.com/auth/ediscovery";
// View your eDiscovery data
pub const AUTH_EDISCOVERY_READONLY: &str = "https://www.googleapis.com/auth/ediscovery.readonly";

// Genomics API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and manage Genomics data
pub const AUTH_GENOMICS: &str = "https://www.googleapis.com/auth/genomics";

// Gmail API, v1
// Read, compose, send, and permanently delete all your email from Gmail
// pub const MAIL_GOOGLE_COM_: &str = "https://mail.google.com/";
// Manage drafts and send emails when you interact with the add-on
pub const AUTH_GMAIL_ADDONS_CURRENT_ACTION_COMPOSE: &str =
    "https://www.googleapis.com/auth/gmail.addons.current.action.compose";
// View your email messages when you interact with the add-on
pub const AUTH_GMAIL_ADDONS_CURRENT_MESSAGE_ACTION: &str =
    "https://www.googleapis.com/auth/gmail.addons.current.message.action";
// View your email message metadata when the add-on is running
pub const AUTH_GMAIL_ADDONS_CURRENT_MESSAGE_METADATA: &str =
    "https://www.googleapis.com/auth/gmail.addons.current.message.metadata";
// View your email messages when the add-on is running
pub const AUTH_GMAIL_ADDONS_CURRENT_MESSAGE_READONLY: &str =
    "https://www.googleapis.com/auth/gmail.addons.current.message.readonly";
// Manage drafts and send emails
pub const AUTH_GMAIL_COMPOSE: &str = "https://www.googleapis.com/auth/gmail.compose";
// Insert mail into your mailbox
pub const AUTH_GMAIL_INSERT: &str = "https://www.googleapis.com/auth/gmail.insert";
// Manage mailbox labels
pub const AUTH_GMAIL_LABELS: &str = "https://www.googleapis.com/auth/gmail.labels";
// View your email message metadata such as labels and headers, but not the email body
pub const AUTH_GMAIL_METADATA: &str = "https://www.googleapis.com/auth/gmail.metadata";
// View and modify but not delete your email
pub const AUTH_GMAIL_MODIFY: &str = "https://www.googleapis.com/auth/gmail.modify";
// View your email messages and settings
pub const AUTH_GMAIL_READONLY: &str = "https://www.googleapis.com/auth/gmail.readonly";
// Send email on your behalf
pub const AUTH_GMAIL_SEND: &str = "https://www.googleapis.com/auth/gmail.send";
// Manage your basic mail settings
pub const AUTH_GMAIL_SETTINGS_BASIC: &str = "https://www.googleapis.com/auth/gmail.settings.basic";
// Manage your sensitive mail settings, including who can manage your mail
pub const AUTH_GMAIL_SETTINGS_SHARING: &str =
    "https://www.googleapis.com/auth/gmail.settings.sharing";

// Google Analytics API, v3
// View and manage your Google Analytics data
// pub const AUTH_ANALYTICS: &str = "https://www.googleapis.com/auth/analytics";
// Edit Google Analytics management entities
pub const AUTH_ANALYTICS_EDIT: &str = "https://www.googleapis.com/auth/analytics.edit";
// Manage Google Analytics Account users by email address
pub const AUTH_ANALYTICS_MANAGE_USERS: &str =
    "https://www.googleapis.com/auth/analytics.manage.users";
// View Google Analytics user permissions
pub const AUTH_ANALYTICS_MANAGE_USERS_READONLY: &str =
    "https://www.googleapis.com/auth/analytics.manage.users.readonly";
// Create a new Google Analytics account along with its default property and view
pub const AUTH_ANALYTICS_PROVISION: &str = "https://www.googleapis.com/auth/analytics.provision";
// View your Google Analytics data
// pub const AUTH_ANALYTICS_READONLY: &str = "https://www.googleapis.com/auth/analytics.readonly";
// Manage Google Analytics user deletion requests
pub const AUTH_ANALYTICS_USER_DELETION: &str =
    "https://www.googleapis.com/auth/analytics.user.deletion";

// Google Classroom API, v1
// View and manage announcements in Google Classroom
pub const AUTH_CLASSROOM_ANNOUNCEMENTS: &str =
    "https://www.googleapis.com/auth/classroom.announcements";
// View announcements in Google Classroom
pub const AUTH_CLASSROOM_ANNOUNCEMENTS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.announcements.readonly";
// Manage your Google Classroom classes
pub const AUTH_CLASSROOM_COURSES: &str = "https://www.googleapis.com/auth/classroom.courses";
// View your Google Classroom classes
pub const AUTH_CLASSROOM_COURSES_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.courses.readonly";
// Manage your course work and view your grades in Google Classroom
pub const AUTH_CLASSROOM_COURSEWORK_ME: &str =
    "https://www.googleapis.com/auth/classroom.coursework.me";
// View your course work and grades in Google Classroom
pub const AUTH_CLASSROOM_COURSEWORK_ME_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.coursework.me.readonly";
// Manage course work and grades for students in the Google Classroom classes you teach and view the course work and grades for classes you administer
pub const AUTH_CLASSROOM_COURSEWORK_STUDENTS: &str =
    "https://www.googleapis.com/auth/classroom.coursework.students";
// View course work and grades for students in the Google Classroom classes you teach or administer
pub const AUTH_CLASSROOM_COURSEWORK_STUDENTS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.coursework.students.readonly";
// See, edit, and create classwork materials in Google Classroom
pub const AUTH_CLASSROOM_COURSEWORKMATERIALS: &str =
    "https://www.googleapis.com/auth/classroom.courseworkmaterials";
// See all classwork materials for your Google Classroom classes
pub const AUTH_CLASSROOM_COURSEWORKMATERIALS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.courseworkmaterials.readonly";
// View your Google Classroom guardians
pub const AUTH_CLASSROOM_GUARDIANLINKS_ME_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly";
// View and manage guardians for students in your Google Classroom classes
pub const AUTH_CLASSROOM_GUARDIANLINKS_STUDENTS: &str =
    "https://www.googleapis.com/auth/classroom.guardianlinks.students";
// View guardians for students in your Google Classroom classes
pub const AUTH_CLASSROOM_GUARDIANLINKS_STUDENTS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly";
// View the email addresses of people in your classes
pub const AUTH_CLASSROOM_PROFILE_EMAILS: &str =
    "https://www.googleapis.com/auth/classroom.profile.emails";
// View the profile photos of people in your classes
pub const AUTH_CLASSROOM_PROFILE_PHOTOS: &str =
    "https://www.googleapis.com/auth/classroom.profile.photos";
// Receive notifications about your Google Classroom data
pub const AUTH_CLASSROOM_PUSH_NOTIFICATIONS: &str =
    "https://www.googleapis.com/auth/classroom.push-notifications";
// Manage your Google Classroom class rosters
pub const AUTH_CLASSROOM_ROSTERS: &str = "https://www.googleapis.com/auth/classroom.rosters";
// View your Google Classroom class rosters
pub const AUTH_CLASSROOM_ROSTERS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.rosters.readonly";
// View your course work and grades in Google Classroom
pub const AUTH_CLASSROOM_STUDENT_SUBMISSIONS_ME_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly";
// View course work and grades for students in the Google Classroom classes you teach or administer
pub const AUTH_CLASSROOM_STUDENT_SUBMISSIONS_STUDENTS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly";
// See, create, and edit topics in Google Classroom
pub const AUTH_CLASSROOM_TOPICS: &str = "https://www.googleapis.com/auth/classroom.topics";
// View topics in Google Classroom
pub const AUTH_CLASSROOM_TOPICS_READONLY: &str =
    "https://www.googleapis.com/auth/classroom.topics.readonly";

// Google Cloud Data Catalog API, v1beta1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Google Cloud Memorystore for Redis API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Google Docs API, v1
// View and manage your Google Docs documents
// pub const AUTH_DOCUMENTS: &str = "https://www.googleapis.com/auth/documents";
// View your Google Docs documents
pub const AUTH_DOCUMENTS_READONLY: &str = "https://www.googleapis.com/auth/documents.readonly";
// See, edit, create, and delete all of your Google Drive files
// pub const AUTH_DRIVE: &str = "https://www.googleapis.com/auth/drive";
// View and manage Google Drive files and folders that you have opened or created with this app
// pub const AUTH_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
// See and download all your Google Drive files
// pub const AUTH_DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";

// Google Identity Toolkit API, v3
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and administer all your Firebase data and settings
// pub const AUTH_FIREBASE: &str = "https://www.googleapis.com/auth/firebase";

// Google OAuth2 API, v2
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";
// See your personal info, including any personal info you've made publicly available
pub const AUTH_USERINFO_PROFILE: &str = "https://www.googleapis.com/auth/userinfo.profile";

// openid	Associate you with your personal info on Google

// Google Play Android Developer API, v3
// View and manage your Google Play Developer account
pub const AUTH_ANDROIDPUBLISHER: &str = "https://www.googleapis.com/auth/androidpublisher";

// Google Play Custom App Publishing API, v1
// View and manage your Google Play Developer account
// pub const AUTH_ANDROIDPUBLISHER: &str = "https://www.googleapis.com/auth/androidpublisher";

// Google Play EMM API, v1
// Manage corporate Android devices
pub const AUTH_ANDROIDENTERPRISE: &str = "https://www.googleapis.com/auth/androidenterprise";

// Google Play Game Management, v1management
// Create, edit, and delete your Google Play Games activity
pub const AUTH_GAMES: &str = "https://www.googleapis.com/auth/games";

// Google Play Game Services, v1
// View and manage its own configuration data in your Google Drive
// pub const AUTH_DRIVE_APPDATA: &str = "https://www.googleapis.com/auth/drive.appdata";
// Create, edit, and delete your Google Play Games activity
// pub const AUTH_GAMES: &str = "https://www.googleapis.com/auth/games";

// Google Play Game Services Publishing API, v1configuration
// View and manage your Google Play Developer account
// pub const AUTH_ANDROIDPUBLISHER: &str = "https://www.googleapis.com/auth/androidpublisher";

// Google Search Console API, v1
// View and manage Search Console data for your verified sites
pub const AUTH_WEBMASTERS: &str = "https://www.googleapis.com/auth/webmasters";
// View Search Console data for your verified sites
pub const AUTH_WEBMASTERS_READONLY: &str = "https://www.googleapis.com/auth/webmasters.readonly";

// Google Sheets API, v4
// See, edit, create, and delete all of your Google Drive files
// pub const AUTH_DRIVE: &str = "https://www.googleapis.com/auth/drive";
// View and manage Google Drive files and folders that you have opened or created with this app
// pub const AUTH_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
// See and download all your Google Drive files
// pub const AUTH_DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
// See, edit, create, and delete your spreadsheets in Google Drive
// pub const AUTH_SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
// View your Google Spreadsheets
pub const AUTH_SPREADSHEETS_READONLY: &str =
    "https://www.googleapis.com/auth/spreadsheets.readonly";

// Google Sign-In

// profile	View your basic profile info

// email	View your email address

// openid	Authenticate using OpenID Connect

// Google Site Verification API, v1
// Manage the list of sites and domains you control
pub const AUTH_SITEVERIFICATION: &str = "https://www.googleapis.com/auth/siteverification";
// Manage your new site verifications with Google
pub const AUTH_SITEVERIFICATION_VERIFY_ONLY: &str =
    "https://www.googleapis.com/auth/siteverification.verify_only";

// Google Slides API, v1
// See, edit, create, and delete all of your Google Drive files
// pub const AUTH_DRIVE: &str = "https://www.googleapis.com/auth/drive";
// View and manage Google Drive files and folders that you have opened or created with this app
// pub const AUTH_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
// See and download all your Google Drive files
// pub const AUTH_DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
// View and manage your Google Slides presentations
pub const AUTH_PRESENTATIONS: &str = "https://www.googleapis.com/auth/presentations";
// View your Google Slides presentations
pub const AUTH_PRESENTATIONS_READONLY: &str =
    "https://www.googleapis.com/auth/presentations.readonly";
// See, edit, create, and delete your spreadsheets in Google Drive
// pub const AUTH_SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
// View your Google Spreadsheets
// pub const AUTH_SPREADSHEETS_READONLY: &str = "https://www.googleapis.com/auth/spreadsheets.readonly";

// Google Workspace Alert Center API, v1beta1
// See and delete your domain's G Suite alerts, and send alert feedback
pub const AUTH_APPS_ALERTS: &str = "https://www.googleapis.com/auth/apps.alerts";

// Google Workspace Reseller API, v1
// Manage users on your domain
pub const AUTH_APPS_ORDER: &str = "https://www.googleapis.com/auth/apps.order";
// Manage users on your domain
pub const AUTH_APPS_ORDER_READONLY: &str = "https://www.googleapis.com/auth/apps.order.readonly";

// Groups Migration API, v1
// Manage messages in groups on your domain
pub const AUTH_APPS_GROUPS_MIGRATION: &str =
    "https://www.googleapis.com/auth/apps.groups.migration";

// Groups Settings API, v1
// View and manage the settings of a G Suite group
pub const AUTH_APPS_GROUPS_SETTINGS: &str = "https://www.googleapis.com/auth/apps.groups.settings";

// HomeGraph API, v1
//
pub const AUTH_HOMEGRAPH: &str = "https://www.googleapis.com/auth/homegraph";

// IAM Service Account Credentials API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Identity and Access Management (IAM) API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Indexing API, v3
// Submit data to Google for indexing
pub const AUTH_INDEXING: &str = "https://www.googleapis.com/auth/indexing";

// Kubernetes Engine API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Library Agent API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Managed Service for Microsoft Active Directory API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Manufacturer Center API, v1
// Manage your product listings for Google Manufacturer Center
pub const AUTH_MANUFACTURERCENTER: &str = "https://www.googleapis.com/auth/manufacturercenter";

// Network Management API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// OS Config API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// OpenID Connect, 1.0

// openid	Authenticate using OpenID Connect

// profile	View your basic profile info

// email	View your email address

// PageSpeed Insights API, v5

// openid	Associate you with your personal info on Google

// People API, v1
// See, edit, download, and permanently delete your contacts
pub const AUTH_CONTACTS: &str = "https://www.googleapis.com/auth/contacts";
// See and download contact info automatically saved in your "Other contacts"
pub const AUTH_CONTACTS_OTHER_READONLY: &str =
    "https://www.googleapis.com/auth/contacts.other.readonly";
// See and download your contacts
pub const AUTH_CONTACTS_READONLY: &str = "https://www.googleapis.com/auth/contacts.readonly";
// See and download your organization's GSuite directory
pub const AUTH_DIRECTORY_READONLY: &str = "https://www.googleapis.com/auth/directory.readonly";
// View your street addresses
pub const AUTH_USER_ADDRESSES_READ: &str = "https://www.googleapis.com/auth/user.addresses.read";
// See and download your exact date of birth
pub const AUTH_USER_BIRTHDAY_READ: &str = "https://www.googleapis.com/auth/user.birthday.read";
// View your email addresses
pub const AUTH_USER_EMAILS_READ: &str = "https://www.googleapis.com/auth/user.emails.read";
// See your gender
pub const AUTH_USER_GENDER_READ: &str = "https://www.googleapis.com/auth/user.gender.read";
// See your education, work history and org info
pub const AUTH_USER_ORGANIZATION_READ: &str =
    "https://www.googleapis.com/auth/user.organization.read";
// See and download your personal phone numbers
pub const AUTH_USER_PHONENUMBERS_READ: &str =
    "https://www.googleapis.com/auth/user.phonenumbers.read";
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";
// See your personal info, including any personal info you've made publicly available
// pub const AUTH_USERINFO_PROFILE: &str = "https://www.googleapis.com/auth/userinfo.profile";

// Photos Library API, v1
// View and manage your Google Photos library
pub const AUTH_PHOTOSLIBRARY: &str = "https://www.googleapis.com/auth/photoslibrary";
// Add to your Google Photos library
pub const AUTH_PHOTOSLIBRARY_APPENDONLY: &str =
    "https://www.googleapis.com/auth/photoslibrary.appendonly";
// Edit the info in your photos, videos, and albums created within this app, including titles, descriptions, and covers
pub const AUTH_PHOTOSLIBRARY_EDIT_APPCREATEDDATA: &str =
    "https://www.googleapis.com/auth/photoslibrary.edit.appcreateddata";
// View your Google Photos library
pub const AUTH_PHOTOSLIBRARY_READONLY: &str =
    "https://www.googleapis.com/auth/photoslibrary.readonly";
// Manage photos added by this app
pub const AUTH_PHOTOSLIBRARY_READONLY_APPCREATEDDATA: &str =
    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata";
// Manage and add to shared albums on your behalf
pub const AUTH_PHOTOSLIBRARY_SHARING: &str =
    "https://www.googleapis.com/auth/photoslibrary.sharing";

// Policy Troubleshooter API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Recommender API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Remote Build Execution API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// SAS Portal API, v1alpha1
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";

// SAS Portal API (Testing), v1alpha1
// View your email address
// pub const AUTH_USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";

// Search Ads 360 API, v2
// View and manage your advertising data in DoubleClick Search
pub const AUTH_DOUBLECLICKSEARCH: &str = "https://www.googleapis.com/auth/doubleclicksearch";

// Search Console API, v3
// View and manage Search Console data for your verified sites
// pub const AUTH_WEBMASTERS: &str = "https://www.googleapis.com/auth/webmasters";
// View Search Console data for your verified sites
// pub const AUTH_WEBMASTERS_READONLY: &str = "https://www.googleapis.com/auth/webmasters.readonly";

// Secret Manager API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Security Command Center API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Service Consumer Management API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Service Management API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// Manage your Google API service configuration
pub const AUTH_SERVICE_MANAGEMENT: &str = "https://www.googleapis.com/auth/service.management";
// View your Google API service configuration
pub const AUTH_SERVICE_MANAGEMENT_READONLY: &str =
    "https://www.googleapis.com/auth/service.management.readonly";

// Service Networking API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// Manage your Google API service configuration
// pub const AUTH_SERVICE_MANAGEMENT: &str = "https://www.googleapis.com/auth/service.management";

// Service Usage API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM_READ_ONLY: &str = "https://www.googleapis.com/auth/cloud-platform.read-only";
// Manage your Google API service configuration
// pub const AUTH_SERVICE_MANAGEMENT: &str = "https://www.googleapis.com/auth/service.management";

// Stackdriver Profiler API, v2
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
// View and write monitoring data for all of your Google and third-party Cloud and API projects
// pub const AUTH_MONITORING: &str = "https://www.googleapis.com/auth/monitoring";
// Publish metric data to your Google Cloud projects
// pub const AUTH_MONITORING_WRITE: &str = "https://www.googleapis.com/auth/monitoring.write";

// Storage Transfer API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// Street View Publish API, v1
// Publish and manage your 360 photos on Google Street View
pub const AUTH_STREETVIEWPUBLISH: &str = "https://www.googleapis.com/auth/streetviewpublish";

// Tag Manager API, v2
// Delete your Google Tag Manager containers
pub const AUTH_TAGMANAGER_DELETE_CONTAINERS: &str =
    "https://www.googleapis.com/auth/tagmanager.delete.containers";
// Manage your Google Tag Manager container and its subcomponents, excluding versioning and publishing
pub const AUTH_TAGMANAGER_EDIT_CONTAINERS: &str =
    "https://www.googleapis.com/auth/tagmanager.edit.containers";
// Manage your Google Tag Manager container versions
pub const AUTH_TAGMANAGER_EDIT_CONTAINERVERSIONS: &str =
    "https://www.googleapis.com/auth/tagmanager.edit.containerversions";
// View and manage your Google Tag Manager accounts
pub const AUTH_TAGMANAGER_MANAGE_ACCOUNTS: &str =
    "https://www.googleapis.com/auth/tagmanager.manage.accounts";
// Manage user permissions of your Google Tag Manager account and container
pub const AUTH_TAGMANAGER_MANAGE_USERS: &str =
    "https://www.googleapis.com/auth/tagmanager.manage.users";
// Publish your Google Tag Manager container versions
pub const AUTH_TAGMANAGER_PUBLISH: &str = "https://www.googleapis.com/auth/tagmanager.publish";
// View your Google Tag Manager container and its subcomponents
pub const AUTH_TAGMANAGER_READONLY: &str = "https://www.googleapis.com/auth/tagmanager.readonly";

// Tasks API, v1
// Create, edit, organize, and delete all your tasks
pub const AUTH_TASKS: &str = "https://www.googleapis.com/auth/tasks";
// View your tasks
pub const AUTH_TASKS_READONLY: &str = "https://www.googleapis.com/auth/tasks.readonly";

// Web Security Scanner API, v1
// View and manage your data across Google Cloud Platform services
// pub const AUTH_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";

// YouTube Analytics API, v2
// Manage your YouTube account
pub const AUTH_YOUTUBE: &str = "https://www.googleapis.com/auth/youtube";
// View your YouTube account
pub const AUTH_YOUTUBE_READONLY: &str = "https://www.googleapis.com/auth/youtube.readonly";
// View and manage your assets and associated content on YouTube
pub const AUTH_YOUTUBEPARTNER: &str = "https://www.googleapis.com/auth/youtubepartner";
// View monetary and non-monetary YouTube Analytics reports for your YouTube content
pub const AUTH_YT_ANALYTICS_MONETARY_READONLY: &str =
    "https://www.googleapis.com/auth/yt-analytics-monetary.readonly";
// View YouTube Analytics reports for your YouTube content
pub const AUTH_YT_ANALYTICS_READONLY: &str =
    "https://www.googleapis.com/auth/yt-analytics.readonly";

// YouTube Data API, v3
// Manage your YouTube account
// pub const AUTH_YOUTUBE: &str = "https://www.googleapis.com/auth/youtube";
// See a list of your current active channel members, their current level, and when they became a member
pub const AUTH_YOUTUBE_CHANNEL_MEMBERSHIPS_CREATOR: &str =
    "https://www.googleapis.com/auth/youtube.channel-memberships.creator";
// See, edit, and permanently delete your YouTube videos, ratings, comments and captions
pub const AUTH_YOUTUBE_FORCE_SSL: &str = "https://www.googleapis.com/auth/youtube.force-ssl";
// View your YouTube account
// pub const AUTH_YOUTUBE_READONLY: &str = "https://www.googleapis.com/auth/youtube.readonly";
// Manage your YouTube videos
pub const AUTH_YOUTUBE_UPLOAD: &str = "https://www.googleapis.com/auth/youtube.upload";
// View and manage your assets and associated content on YouTube
// pub const AUTH_YOUTUBEPARTNER: &str = "https://www.googleapis.com/auth/youtubepartner";
// View private information of your YouTube channel relevant during the audit process with a YouTube partner
pub const AUTH_YOUTUBEPARTNER_CHANNEL_AUDIT: &str =
    "https://www.googleapis.com/auth/youtubepartner-channel-audit";

// YouTube Reporting API, v1
// View monetary and non-monetary YouTube Analytics reports for your YouTube content
// pub const AUTH_YT_ANALYTICS_MONETARY_READONLY: &str = "https://www.googleapis.com/auth/yt-analytics-monetary.readonly";
// View YouTube Analytics reports for your YouTube content
// pub const AUTH_YT_ANALYTICS_READONLY: &str = "https://www.googleapis.com/auth/yt-analytics.readonly";
