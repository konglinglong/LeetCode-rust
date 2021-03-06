/// ```rust,ignore
/// 119. 杨辉三角 II
/// 给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
///
/// 在杨辉三角中，每个数是它左上方和右上方的数的和。
///
/// 示例:
///
/// 输入: 3
/// 输出: [1,3,3,1]
///
/// 进阶：
///
/// 你可以优化你的算法到 O(k) 空间复杂度吗？
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/pascals-triangle-ii
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut vec:Vec<Vec<i32>> = Vec::new();
    for i in 0..row_index + 1 {
        vec.push(vec![1; (i + 1) as usize]);
        if i > 1 {
            for j in 1..i {
                vec[i as usize][j as usize] = vec[(i - 1) as usize][(j - 1) as usize] + vec[(i - 1) as usize][j as usize];
            }
        }
    }
    vec[row_index as usize].clone()
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_get_row()
    {
        assert_eq!(get_row(3), vec![1,3,3,1]);
    }
}

