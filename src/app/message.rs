//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::router::Routers;

#[derive(Debug, Clone)]
pub enum RouteMessages {
    Next(Routers),
    Back,
}

#[derive(Debug, Clone)]
pub enum GlobalMessages {
    Route(RouteMessages),
}
