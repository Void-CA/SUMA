#[macro_export]
macro_rules! float_uwgraph {
    (
        $(
            $from:tt => {
                $( $to:tt : $weight:expr ),* $(,)?
            }
        ),* $(,)?
    ) => {{
        let mut g = $crate::UndirectedWeightedGraph::new_float();

        $(
            $(
                g.add_edge(
                    float_uwgraph!(@node $from),
                    float_uwgraph!(@node $to),
                    $weight
                );
            )*
        )*

        g
    }};

    (@node $n:ident) => {
        stringify!($n)
    };

    (@node $n:expr) => {
        $n
    };
}


#[macro_export]
macro_rules! uwgraph {
    (
        $(
            $from:tt => {
                $( $to:tt : $weight:expr ),* $(,)?
            }
        ),* $(,)?
    ) => {{
        let mut g = $crate::UndirectedWeightedGraph::new();

        $(
            $(
                g.add_edge(
                    uwgraph!(@node $from),
                    uwgraph!(@node $to),
                    $weight
                );
            )*
        )*

        g
    }};

    (@node $n:ident) => {
        stringify!($n)
    };

    (@node $n:expr) => {
        $n
    };
}
