use variable::Variable;

type Clause<'a> = (Option<&'a Variable>, Option<&'a Variable>, Option<&'a Variable>);