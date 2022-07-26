pub fn find_content_children(children: &mut [u32], cookies: &mut [u32]) -> u32 {
    children.sort();
    cookies.sort();
    let mut child = 0;
    let mut cookie = 0;

    while child < children.len() && cookie < cookies.len() {
        if children[child] <= cookies[cookie] {
            child += 1;
        }

        cookie += 1;
    }

    return child as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children() {
        let mut children = vec![1, 2, 3];
        let mut cookies = vec![1, 2];
        assert_eq!(2, find_content_children(&mut children, &mut cookies));
    }
}
