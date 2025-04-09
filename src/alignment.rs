/*Übung 1

Nehmen Sie eine Referenz- und eine Querysequenz sowie den CIGAR-String, der das Alignment spezifiziert (z.B. von Abb. 1). In unserem Beispiel wären die drei Argumente also

    Query (read_4): GCATCTGTTG
    Ref: GCATACTGTTG
    CIGAR: 4M1D6M

Das Ziel ist es, eine Funktion zu schreiben, welche das entsprechende Alignment ausgibt, also in unserem Beispiel

GCAT-CTGTTG
|||| ||||||
GCATACTGTTG*/

pub fn cigar_to_alignment(query: &str, reference: &str, cigar: &str) -> Vec<String> {
    let mut aligned_query = String::new();
    let mut alignment_bar = String::new();
    let mut aligned_ref = String::new();

    let mut query_pos = 0;
    let mut ref_pos = 0;

    let mut num_buf = String::new(); // to keep track of numbers (digits) before operation

    for c in cigar.chars() {
        if c.is_digit(10) { // check if character is a digit
            num_buf.push(c);
        } else {
            let n: usize = num_buf.parse().unwrap(); // put digits together as number
            num_buf.clear();

            match c {
                'M' => {
                    for _ in 0..n {
                        let q = query.chars().nth(query_pos).unwrap(); // nth() get the n-th element, consume elements before it
                        let r = reference.chars().nth(ref_pos).unwrap();

                        aligned_query.push(q);
                        aligned_ref.push(r);
                        alignment_bar.push(if q == r { '|' } else { ' ' });

                        query_pos += 1;
                        ref_pos += 1;
                    }
                },
                'D' => {
                    for _ in 0..n {
                        aligned_query.push('-');
                        let r = reference.chars().nth(ref_pos).unwrap();
                        aligned_ref.push(r);
                        alignment_bar.push(' ');
                        ref_pos += 1;
                    }
                },
                'I' => {
                    for _ in 0..n {
                        let q = query.chars().nth(query_pos).unwrap();
                        aligned_query.push(q);
                        aligned_ref.push('-');
                        alignment_bar.push(' ');
                        query_pos += 1;
                    }
                },
                _ => panic!("Unsupported CIGAR op: {}", c),
            }
        }
    }

    vec![aligned_query, alignment_bar, aligned_ref]
}



#[cfg(test)]

mod test {
    use super::cigar_to_alignment;

    #[test]

    fn test_cigar_to_alignment (){

        let query = "GCATCTGTTG";
        let reference = "GCATACTGTTG";
        let cigar = "4M1D6M";

        let result = cigar_to_alignment(query, reference, cigar);
        let expected = vec!["GCAT-CTGTTG", "|||| ||||||", "GCATACTGTTG"];

        assert_eq!(result, expected);

        for line in result{
            println!("{}", line);
        }

    }
}