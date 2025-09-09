//! List encoding conversions

#![allow(missing_docs)]

use crate::data::num::convert::*;
use crate::term::DeBruijnTerm::*;
use crate::term::{abs, app, DeBruijnTerm};

macro_rules! make_trait {
    ($trait_name:ident, $function_name:ident) => {
        pub trait $trait_name {
            #[doc = "Performs the conversion."]
            fn $function_name(self) -> DeBruijnTerm;
        }
    };
}

make_trait!(IntoPairList, into_pair_list);
make_trait!(IntoChurchList, into_church);
make_trait!(IntoScottList, into_scott);
make_trait!(IntoParigotList, into_parigot);

impl IntoPairList for Vec<DeBruijnTerm> {
    fn into_pair_list(self) -> DeBruijnTerm {
        let mut ret = abs!(2, Var(1));

        for t in self.into_iter().rev() {
            ret = abs(app!(Var(1), t, ret))
        }

        ret
    }
}

impl IntoChurchList for Vec<DeBruijnTerm> {
    fn into_church(self) -> DeBruijnTerm {
        let mut ret = Var(2);

        for t in self.into_iter().rev() {
            ret = app!(Var(1), t, ret);
        }

        abs!(2, ret)
    }
}

impl<T: IntoChurchNum> IntoChurchList for Vec<T> {
    fn into_church(self) -> DeBruijnTerm {
        self.into_iter()
            .map(|t| t.into_church())
            .collect::<Vec<DeBruijnTerm>>()
            .into_church()
    }
}

impl IntoScottList for Vec<DeBruijnTerm> {
    fn into_scott(self) -> DeBruijnTerm {
        let mut ret = abs!(2, Var(2));

        for t in self.into_iter().rev() {
            ret = abs!(2, app!(Var(1), t, ret));
        }

        ret
    }
}

impl<T: IntoScottNum> IntoScottList for Vec<T> {
    fn into_scott(self) -> DeBruijnTerm {
        self.into_iter()
            .map(|t| t.into_scott())
            .collect::<Vec<DeBruijnTerm>>()
            .into_scott()
    }
}

impl IntoParigotList for Vec<DeBruijnTerm> {
    fn into_parigot(self) -> DeBruijnTerm {
        let mut ret = abs!(2, Var(2));

        for t in self.into_iter().rev() {
            ret = abs!(
                2,
                app!(
                    Var(1),
                    t,
                    ret.clone(),
                    ret.unabs().and_then(|r| r.unabs()).unwrap()
                )
            );
        }

        ret
    }
}

impl<T: IntoParigotNum> IntoParigotList for Vec<T> {
    fn into_parigot(self) -> DeBruijnTerm {
        self.into_iter()
            .map(|t| t.into_parigot())
            .collect::<Vec<DeBruijnTerm>>()
            .into_parigot()
    }
}
