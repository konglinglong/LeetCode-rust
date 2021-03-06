/// 
/// 比较两个版本号 version1 和 version2。
/// 如果 version1 > version2 返回 1，如果 version1 < version2 返回 -1， 除此之外返回 0。
/// 
/// 你可以假设版本字符串非空，并且只包含数字和 . 字符。
/// 
///  . 字符不代表小数点，而是用于分隔数字序列。
/// 
/// 例如，2.5 不是“两个半”，也不是“差一半到三”，而是第二版中的第五个小版本。
/// 
/// 你可以假设版本号的每一级的默认修订版号为 0。例如，版本号 3.4 的第一级（大版本）和第二级（小版本）修订号分别为 3 和 4。其第三级和第四级修订号均为 0。
///  
/// 
/// 示例 1:
/// 
/// 输入: version1 = "0.1", version2 = "1.1"
/// 输出: -1
/// 
/// 示例 2:
/// 
/// 输入: version1 = "1.0.1", version2 = "1"
/// 输出: 1
/// 
/// 示例 3:
/// 
/// 输入: version1 = "7.5.2.4", version2 = "7.5.3"
/// 输出: -1
/// 
/// 示例 4：
/// 
/// 输入：version1 = "1.01", version2 = "1.001"
/// 输出：0
/// 解释：忽略前导零，“01” 和 “001” 表示相同的数字 “1”。
/// 
/// 示例 5：
/// 
/// 输入：version1 = "1.0", version2 = "1.0.0"
/// 输出：0
/// 解释：version1 没有第三级修订号，这意味着它的第三级修订号默认为 “0”。
/// 

pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1: Vec<&str> = version1.split(".").collect();
    let v2: Vec<&str> = version2.split(".").collect();
    
    //println!("v1 = {:?}", v1);
    //println!("v2 = {:?}", v2);
    
    let v1_len = v1.len();
    let v2_len = v2.len();
    let size = if v1_len > v2_len { v1_len } else { v2_len };
    //println!("size = {}", size);
    
    for i in 0..size {
    	let mut num1 = 0;
    	let mut num2 = 0;
    	
    	if i < v1_len {
    		num1 = v1[i].parse::<i32>().unwrap();
    	}
    	if i < v2_len {
    		num2 = v2[i].parse::<i32>().unwrap();
    	}
    	if num1 > num2 {
    		return 1;
    	} else if num1 < num2 {
    		return -1;
    	}
    }
    
     0
}

#[cfg(test)]
mod test
{
    use super::compare_version;

    #[test]
    fn test_compare_version()
    {
        assert_eq!(compare_version("0.1".to_string(), "1.1".to_string()), -1);
        assert_eq!(compare_version("1.0.1".to_string(), "1.0".to_string()), 1);
    }
}