// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) static FN_DEF_SRCS: [&'static str; 23] = [
    "{fn v1 = []}",
    "{fn v2 = []}",
    "{fn v3 = []}",
    "{fn v4 = []}",
    "{fn false = [drop]}",
    "{fn true = [swap drop]}",
    "{fn or = clone apply}",
    "{fn quote2 = quote swap quote swap compose}",
    "{fn quote3 = quote2 swap quote swap compose}",
    "{fn rotate3 = quote2 swap quote compose apply}",
    "{fn rotate4 = quote3 swap quote compose apply}",
    "{fn compose2 = compose}",
    "{fn compose3 = compose compose2}",
    "{fn compose4 = compose compose3}",
    "{fn compose5 = compose compose4}",
    "{fn n0 = [drop]}",
    "{fn n1 = [[clone] n0 apply [compose] n0 apply apply]}",
    "{fn n2 = [[clone] n1 apply [compose] n1 apply apply]}",
    "{fn n3 = [[clone] n2 apply [compose] n2 apply apply]}",
    "{fn n4 = [[clone] n3 apply [compose] n3 apply apply]}",
    "{fn succ = quote [apply] compose [[clone]] swap clone [[compose]] swap [apply] compose5}",
    "{fn add = [succ] swap apply}",
    "{fn mul = n0 rotate3 quote [add] compose rotate3 apply}",
];
