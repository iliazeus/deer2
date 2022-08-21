macro_rules! do_3 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        $lhs.0 $op $rhs;
        $lhs.1 $op $rhs;
        $lhs.2 $op $rhs;
   };

    ($lhs:ident.i $op:tt $rhs:ident.i) => {
        $lhs.0 $op $rhs.0;
        $lhs.1 $op $rhs.1;
        $lhs.2 $op $rhs.2;
    };
}

macro_rules! self_from_3 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        Self(
            $lhs.0 $op $rhs,
            $lhs.1 $op $rhs,
            $lhs.2 $op $rhs,
        )
    };

    ($lhs:ident.i $op:tt $rhs:ident.i) => {
        Self(
            $lhs.0 $op $rhs.0,
            $lhs.1 $op $rhs.1,
            $lhs.2 $op $rhs.2,
        )
    };

    ($arg:expr) => {
        Self($arg, $arg, $arg)
    };
}
