#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/** THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, Credentials};
use failure::Error;
use serde_json::Value;
use crate::util::urlencode;

/// The index service is responsible for indexing tasks. The service ensures that
/// tasks can be located by user-defined names.
/// 
/// As described in the service documentation, tasks are typically indexed via Pulse
/// messages, so the most common use of API methods is to read from the index.
pub struct Index (Client);

#[allow(non_snake_case)]
impl Index {
    pub fn new(root_url: &str, credentials: Option<Credentials>) -> Result<Self, Error> {
        Ok(Self(Client::new(root_url, "index", "v1", credentials)?))
    }

    /// Ping Server
    /// 
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let path = "ping";
        let query = None;
        let body = None;

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Find Indexed Task
    /// 
    /// Find a task by index path, returning the highest-rank task with that path. If no
    /// task exists for the given path, this API end-point will respond with a 404 status.
    pub async fn findTask(&self, indexPath: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("task/{}", urlencode(indexPath));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// List Namespaces
    /// 
    /// List the namespaces immediately under a given namespace.
    /// 
    /// This endpoint
    /// lists up to 1000 namespaces. If more namespaces are present, a
    /// `continuationToken` will be returned, which can be given in the next
    /// request. For the initial request, the payload should be an empty JSON
    /// object.
    pub async fn listNamespaces(&self, namespace: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("namespaces/{}", urlencode(namespace));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// List Tasks
    /// 
    /// List the tasks immediately under a given namespace.
    /// 
    /// This endpoint
    /// lists up to 1000 tasks. If more tasks are present, a
    /// `continuationToken` will be returned, which can be given in the next
    /// request. For the initial request, the payload should be an empty JSON
    /// object.
    /// 
    /// **Remark**, this end-point is designed for humans browsing for tasks, not
    /// services, as that makes little sense.
    pub async fn listTasks(&self, namespace: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("tasks/{}", urlencode(namespace));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Insert Task into Index
    /// 
    /// Insert a task into the index.  If the new rank is less than the existing rank
    /// at the given index path, the task is not indexed but the response is still 200 OK.
    /// 
    /// Please see the introduction above for information
    /// about indexing successfully completed tasks automatically using custom routes.
    pub async fn insertTask(&self, namespace: &str, payload: &Value) -> Result<Value, Error> {
        let method = "PUT";
        let path = format!("task/{}", urlencode(namespace));
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Get Artifact From Indexed Task
    /// 
    /// Find a task by index path and redirect to the artifact on the most recent
    /// run with the given `name`.
    /// 
    /// Note that multiple calls to this endpoint may return artifacts from differen tasks
    /// if a new task is inserted into the index between calls. Avoid using this method as
    /// a stable link to multiple, connected files if the index path does not contain a
    /// unique identifier.  For example, the following two links may return unrelated files:
    /// * https://tc.example.com/api/index/v1/task/some-app.win64.latest.installer/artifacts/public/installer.exe`
    /// * https://tc.example.com/api/index/v1/task/some-app.win64.latest.installer/artifacts/public/debug-symbols.zip`
    /// 
    /// This problem be remedied by including the revision in the index path or by bundling both
    /// installer and debug symbols into a single artifact.
    /// 
    /// If no task exists for the given index path, this API end-point responds with 404.
    pub async fn findArtifactFromTask(&self, indexPath: &str, name: &str) -> Result<(), Error> {
        let method = "GET";
        let path = format!("task/{}/artifacts/{}", urlencode(indexPath), urlencode(name));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }
}
