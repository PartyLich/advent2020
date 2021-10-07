//! Solutions to 2020 day 4, part 2
//! --- Day 4: Passport Processing ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byr() {
        let msg = "should create a BirthYear";
        let expected = Some(BirthYear(2002));
        let actual = BirthYear::new("2002");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a BirthYear";
        let expected = None;
        let actual = BirthYear::new("2003");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn hgt() {
        let msg = "should create a Height";
        let expected = Some(Height());
        let actual = Height::new("60in");
        assert_eq!(actual, expected, "{}", msg);
        let actual = Height::new("190cm");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a Height";
        let expected = None;
        let actual = Height::new("190in");
        assert_eq!(actual, expected, "{}", msg);
        let actual = Height::new("190");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn hcl() {
        let msg = "should create a HairColor";
        let expected = Some(HairColor("#123abc".to_string()));
        let actual = HairColor::new("#123abc");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a HairColor";
        let expected = None;
        let actual = HairColor::new("#123abz");
        assert_eq!(actual, expected, "{}", msg);
        let actual = HairColor::new("123abc");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn ecl() {
        let msg = "should create a EyeColor";
        let expected = Some(EyeColor());
        let actual = EyeColor::new("brn");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a EyeColor";
        let expected = None;
        let actual = EyeColor::new("wat");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn pid() {
        let msg = "should create a PassportID";
        let expected = Some(PassportID("000000001".to_string()));
        let actual = PassportID::new("000000001");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a PassportID";
        let expected = None;
        let actual = PassportID::new("0123456789");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should count the number of valid passports, with optional cid";
        let expected = 4;
        let actual = two("input/4-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
