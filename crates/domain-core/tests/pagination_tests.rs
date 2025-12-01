use domain_core::pagination::{DEFAULT_PAGE_SIZE, Page, PageResult};

#[test]
fn page_has_more_logic() {
    let page = PageResult::new(vec![1, 2, 3], 10, 0, Some(3), Some(0));
    assert!(page.has_more());
    assert_eq!(page.total_pages(), 4);

    let last = PageResult::new(vec![7, 8, 9, 10], 10, 3, Some(4), Some(0));
    assert!(!last.has_more());
    assert_eq!(last.total_pages(), 3);
}

#[test]
fn empty_page_helper() {
    let empty: PageResult<u8> = PageResult::empty(Some(20), 0, Some(0));
    assert_eq!(empty.items().len(), 0);
    assert_eq!(empty.total_count(), 0);
    assert_eq!(empty.page_size(), 20);
    assert_eq!(empty.page_index(), 0);
    assert_eq!(empty.total_pages(), 0);
    assert!(!empty.has_previous_page());
}

#[test]
fn defaults_page_size_when_none() {
    let page = PageResult::new(vec![42], 1, 0, None, None);
    assert_eq!(page.page_size(), DEFAULT_PAGE_SIZE);

    let empty: PageResult<u8> = PageResult::empty(None, 1, None);
    assert_eq!(empty.page_size(), DEFAULT_PAGE_SIZE);
}

#[test]
fn builder_api_works() {
    let page = PageResult::builder(vec![1, 2], 2)
        .page_index(1)
        .page_size(50)
        .index_from(1)
        .build();

    assert_eq!(page.items(), &[1, 2]);
    assert_eq!(page.total_count(), 2);
    assert_eq!(page.page_index(), 1);
    assert_eq!(page.page_size(), 50);
    assert_eq!(page.index_from(), 1);

    let default_page = PageResult::builder(Vec::<i32>::new(), 0).build();
    assert_eq!(default_page.page_size(), DEFAULT_PAGE_SIZE);
}
