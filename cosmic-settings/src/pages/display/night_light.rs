// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{Message, NightLight};
use crate::pages;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{dropdown, list_column, row, settings, slider, text, toggler};
use cosmic::{Apply, Element};
use cosmic_settings_config::night_light::{MAX_TEMPERATURE, MIN_TEMPERATURE};
use std::sync::LazyLock;

static HOUR_OPTIONS: LazyLock<Vec<String>> =
    LazyLock::new(|| (0..24).map(|hour| format!("{hour:02}")).collect());

static MINUTE_OPTIONS: LazyLock<Vec<String>> =
    LazyLock::new(|| [0, 15, 30, 45].iter().map(|minute| format!("{minute:02}")).collect());

static MINUTE_VALUES: [u16; 4] = [0, 15, 30, 45];

fn time_selectors(
    hour: u16,
    minute: u16,
    on_hour: impl Fn(u16) -> Message + Send + Sync + 'static,
    on_minute: impl Fn(u16) -> Message + Send + Sync + 'static,
) -> cosmic::Element<'static, Message> {
    row::with_capacity(2)
        .align_y(Alignment::Center)
        .push(dropdown(
            &HOUR_OPTIONS[..],
            Some(hour as usize),
            move |selected| on_hour(selected as u16),
        ))
        .push(text::body(":"))
        .push(dropdown(
            &MINUTE_OPTIONS[..],
            Some((minute / 15) as usize),
            move |selected| on_minute(MINUTE_VALUES[selected]),
        ))
        .into()
}

impl super::Page {
    pub fn night_light_view(&self) -> Element<'_, pages::Message> {
        let config = &self.night_light_config;

        let mut section = list_column();

        section = section.add(
            settings::item::builder(fl!("night-light"))
                .description(fl!("night-light", "desc"))
                .control(toggler(config.enabled).on_toggle(|enable| {
                    Message::NightLight(NightLight::Toggle(enable))
                })),
        );

        if config.enabled {
            let temperature = config.temperature.clamp(MIN_TEMPERATURE, MAX_TEMPERATURE);

            section = section.add(
                settings::item::builder(fl!("night-light", "temperature"))
                    .control(
                        row::with_capacity(2)
                            .align_y(Alignment::Center)
                            .spacing(12)
                            .push(
                                slider(
                                    MIN_TEMPERATURE..=MAX_TEMPERATURE,
                                    temperature,
                                    |value| Message::NightLight(NightLight::Temperature(value)),
                                )
                                .step(50.0_f32)
                                .width(Length::Fixed(220.0)),
                            )
                            .push(text::body(format!("{} K", temperature.round() as i32))),
                    ),
            );

            section = section.add(
                settings::item::builder(fl!("night-light", "auto"))
                    .control(toggler(config.auto_schedule).on_toggle(|enable| {
                        Message::NightLight(NightLight::AutoSchedule(enable))
                    })),
            );

            if !config.auto_schedule {
                let start_hour = config.schedule_start_minutes / 60;
                let start_minute = config.schedule_start_minutes % 60;
                let end_hour = config.schedule_end_minutes / 60;
                let end_minute = config.schedule_end_minutes % 60;

                section = section.add(
                    settings::item::builder(fl!("night-light", "start")).control(time_selectors(
                        start_hour,
                        start_minute,
                        move |hour| {
                            Message::NightLight(NightLight::ScheduleStart(hour, start_minute))
                        },
                        move |minute| {
                            Message::NightLight(NightLight::ScheduleStart(start_hour, minute))
                        },
                    )),
                );

                section = section.add(
                    settings::item::builder(fl!("night-light", "end")).control(time_selectors(
                        end_hour,
                        end_minute,
                        move |hour| Message::NightLight(NightLight::ScheduleEnd(hour, end_minute)),
                        move |minute| {
                            Message::NightLight(NightLight::ScheduleEnd(end_hour, minute))
                        },
                    )),
                );
            }
        }

        section
            .apply(Element::from)
            .map(crate::pages::Message::Displays)
    }
}
