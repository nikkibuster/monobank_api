pub(crate) trait FormattedAmount {
    fn format(&mut self);
}

impl FormattedAmount for Vec<super::models::statement::Statement> {
    fn format(&mut self) {
        self.iter_mut().for_each(|st| {
            st.amount /= 100f64;
            st.operation_amount /= 100f64;
            st.balance /= 100f64
        });
    }
}
