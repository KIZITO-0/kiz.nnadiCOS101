use std::collections::HashMap;

fn build_mappings() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut mappings = HashMap::new();

    mappings.insert(
        "Lawyer",
        HashMap::from([
            ("Paralegal", "APS 1-2"),
            ("Junior Associate", "APS 3-5"),
            ("Associate", "APS 5-8"),
            ("Senior Associate 1-2", "EL1 8-10"),
            ("Senior Associate 3-4", "EL2 10-13"),
            ("Partner", "SES"),
        ]),
    );

    mappings.insert(
        "Office Admin",
        HashMap::from([
            ("Intern", "APS 1-2"),
            ("Administrator", "APS 3-5"),
            ("Senior Administrator", "APS 5-8"),
            ("Office Manager", "EL1 8-10"),
            ("Director", "EL2 10-13"),
            ("CEO", "SES"),
        ]),
    );

    mappings.insert(
        "Academic",
        HashMap::from([
            ("Research Assistant", "APS 3-5"),
            ("PhD Candidate", "APS 5-8"),
            ("Post-Doc Researcher", "EL1 8-10"),
            ("Senior Lecturer", "EL2 10-13"),
            ("Dean", "SES"),
        ]),
    );

    mappings.insert(
        "Teacher",
        HashMap::from([
            ("Placement", "APS 1-2"),
            ("Classroom Teacher", "APS 3-5"),
            ("Senior Teacher", "APS 5-8"),
            ("Leading Teacher", "EL1 8-10"),
            ("Deputy Principal", "EL2 10-13"),
            ("Principal", "SES"),
        ]),
    );

    mappings
}

fn get_aps_level< 'a> (profession: & 'a str, role: & 'a  str) -> Option<& 'a str> {
    let mappings = build_mappings();

    mappings
        .get(profession)
        .and_then(|roles| roles.get(role))
        .copied()
}

fn main() {
    let profession = "Lawyer";
    let role = "Associate";

    match get_aps_level(profession, role) {
        Some(level) => println!("APS Level: {}", level),
        None => println!("Invalid profession or role"),
    }
}

