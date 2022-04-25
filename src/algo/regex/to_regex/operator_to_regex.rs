use super::super::super::structures::{Operator, TreeNode};


pub fn operator_to_regex(operator: &Operator, left_subtree: &str, right_subtree: &str, regex: bool) -> String {
    let mut result = String::new();

    let escape = |_result: &mut String| {
        if regex { 
            _result.push_str(r"\");
        };
    };
    escape(&mut result);
    result.push_str(&operator.to_string());

    escape(&mut result);
    result.push_str(r"(");
    result.push_str(left_subtree);
    escape(&mut result);
    result.push_str(r")");

    escape(&mut result);
    result.push_str(r"(");
    result.push_str(right_subtree);
    escape(&mut result);
    result.push_str(r")");

    return result;
}

#[cfg(test)]
mod tests {

    use super::{Operator, operator_to_regex};

    #[test]
    fn test_operator_to_string(){
        let result = operator_to_regex(&Operator {
            label: b'+',
        }, "left", "right", false);

        assert_eq!(result, r"\+\(left\)\(right\)")
    }
}