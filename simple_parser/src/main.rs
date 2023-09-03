#[derive(Debug)]
enum Operator {
    Equal,
    NotEqual,
    SimilarTo,
    StartsWith,
    EndsWith
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, char1) in s1.chars().enumerate() {
        for (j, char2) in s2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

fn are_strings_similar(s1: &str, s2: &str, threshold: usize) -> bool {
    let distance = levenshtein_distance(s1, s2);
    distance <= threshold
}

struct QueryParser;

impl QueryParser {
    fn parse_operator(&self, operator: &str) -> Operator {
        match operator {
            "=" => Operator::Equal,
            "!=" => Operator::NotEqual,
            "~=" => Operator::SimilarTo,
            "^=" => Operator::StartsWith,
            "$=" => Operator::EndsWith,
            _ => Operator::Equal,
        }
    }

    fn parse_query(&self, expression: &str) -> bool {
        const SEPARATOR: char = ' ';
        let mut parts = expression.split(SEPARATOR);
        
        let mut operator: Operator = Operator::Equal;
        let mut first_value = ""; 
        let mut second_value = "";

        let length_of_parts = parts.clone().count();

        match length_of_parts {
            2 => {
                first_value = match parts.next() {
                    Some(s) => s,
                    None => ""
                };
        
                second_value = match parts.next() {
                    Some(s) => s,
                    None => ""
                };
            },
            3 => {
                first_value = match parts.next() {
                    Some(s) => s,
                    None => ""
                };
        
                operator = match parts.next() {
                    Some(op) => self.parse_operator(&op),
                    None => Operator::Equal
                };
        
                second_value = match parts.next() {
                    Some(s) => s,
                    None => ""
                };
            },
            _ => {
                println!("Não reconheci nenhuma expressão válida!");
                return false;
            }
        }

        let expression_result = match operator {
            Operator::Equal => first_value == second_value,
            Operator::NotEqual => first_value != second_value,
            Operator::EndsWith => first_value.ends_with(second_value),
            Operator::StartsWith => first_value.starts_with(second_value),
            Operator::SimilarTo => are_strings_similar(first_value, second_value, 5),
        };

        return expression_result;
    }
}

fn main() {
    let similar_expr_false = QueryParser.parse_query(String::from("odylon ~= reni").as_str());
    let similar_expr_true = QueryParser.parse_query(String::from("renilson ~= reni").as_str());
    
    let equal_operation_without_operator_true = QueryParser.parse_query(String::from("reni reni").as_str());
    let equal_operation_without_operator_false = QueryParser.parse_query(String::from("renilson reni").as_str());

    let invalid_expression = QueryParser.parse_query("abc = edf = fd");
    

    assert_eq!(similar_expr_false, false);
    assert_eq!(similar_expr_true, true);


    assert_eq!(equal_operation_without_operator_false, false);
    assert_eq!(equal_operation_without_operator_true, true);

    assert_eq!(invalid_expression, false);
}