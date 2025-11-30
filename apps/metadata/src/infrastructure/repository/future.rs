use std::future::Future;
use std::pin::Pin;

use domain_core::domain_error::DomainError;

pub type RepoFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, DomainError>> + Send + 'a>>;

pub fn repo_future<'a, T, Fut>(future: Fut) -> RepoFuture<'a, T>
where
    Fut: Future<Output = Result<T, DomainError>> + Send + 'a,
{
    Box::pin(future)
}

