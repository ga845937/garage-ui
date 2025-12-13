//! Pagination utilities
//!
//! 通用分頁工具

/// 分頁結果
#[derive(Debug, Clone)]
pub struct PaginationResult<T> {
    /// 當前頁資料
    pub data: Vec<T>,
    /// 總筆數（分頁前）
    pub total: usize,
    /// 當前頁碼
    pub page: usize,
    /// 每頁筆數
    pub page_size: usize,
    /// 總頁數
    pub total_pages: usize,
}

impl<T> PaginationResult<T> {
    pub fn new(data: Vec<T>, total: usize, page: usize, page_size: usize) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            (total + page_size - 1) / page_size
        };
        
        Self {
            data,
            total,
            page,
            page_size,
            total_pages,
        }
    }

    /// 是否有下一頁
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// 是否有上一頁
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}

/// 對切片進行分頁
/// 
/// # Arguments
/// * `items` - 要分頁的資料
/// * `page` - 頁碼（從 1 開始）
/// * `page_size` - 每頁筆數
/// 
/// # Returns
/// 分頁後的資料（Vec）
/// 
/// # Examples
/// ```
/// use garage_ui::shared::paginate;
/// 
/// let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let page1 = paginate(&items, 1, 3);
/// assert_eq!(page1, vec![1, 2, 3]);
/// 
/// let page2 = paginate(&items, 2, 3);
/// assert_eq!(page2, vec![4, 5, 6]);
/// ```
pub fn paginate<T: Clone>(items: &[T], page: usize, page_size: usize) -> Vec<T> {
    if page == 0 || page_size == 0 {
        return vec![];
    }
    let start = (page - 1) * page_size;
    if start >= items.len() {
        return vec![];
    }
    let end = (start + page_size).min(items.len());
    items[start..end].to_vec()
}

/// 對切片進行分頁，返回完整的分頁結果
pub fn paginate_with_info<T: Clone>(items: &[T], page: usize, page_size: usize) -> PaginationResult<T> {
    let total = items.len();
    let data = paginate(items, page, page_size);
    PaginationResult::new(data, total, page, page_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate_first_page() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = paginate(&items, 1, 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_paginate_middle_page() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = paginate(&items, 2, 3);
        assert_eq!(result, vec![4, 5, 6]);
    }

    #[test]
    fn test_paginate_last_page_partial() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = paginate(&items, 4, 3);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_paginate_beyond_last_page() {
        let items = vec![1, 2, 3, 4, 5];
        let result = paginate(&items, 10, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_paginate_page_zero() {
        let items = vec![1, 2, 3];
        let result = paginate(&items, 0, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_paginate_page_size_zero() {
        let items = vec![1, 2, 3];
        let result = paginate(&items, 1, 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_pagination_result() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = paginate_with_info(&items, 2, 3);
        
        assert_eq!(result.data, vec![4, 5, 6]);
        assert_eq!(result.total, 10);
        assert_eq!(result.page, 2);
        assert_eq!(result.page_size, 3);
        assert_eq!(result.total_pages, 4);
        assert!(result.has_next());
        assert!(result.has_prev());
    }

    #[test]
    fn test_pagination_result_first_page() {
        let items = vec![1, 2, 3, 4, 5];
        let result = paginate_with_info(&items, 1, 2);
        
        assert!(!result.has_prev());
        assert!(result.has_next());
    }

    #[test]
    fn test_pagination_result_last_page() {
        let items = vec![1, 2, 3, 4, 5];
        let result = paginate_with_info(&items, 3, 2);
        
        assert!(result.has_prev());
        assert!(!result.has_next());
    }
}
