//! MoFA FM Screen UI Design
//!
//! Contains the live_design! DSL block defining the UI layout and styling.

use makepad_widgets::*;

use super::MoFaFMScreen;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use mofa_widgets::theme::*;
    use mofa_widgets::participant_panel::ParticipantPanel;
    use mofa_widgets::log_panel::LogPanel;
    use crate::mofa_hero::MofaHero;

    // Local layout constants (colors imported from theme)
    SECTION_SPACING = 12.0
    PANEL_RADIUS = 4.0
    PANEL_PADDING = 12.0

    // Tab button style
    TabButton = <View> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                // Light: transparent -> slate-100 (hover) -> white (selected)
                // Dark: transparent -> slate-700 (hover) -> slate-600 (selected)
                let light_normal = vec4(0.0, 0.0, 0.0, 0.0);
                let light_hover = (SLATE_100);
                let light_selected = (WHITE);
                let dark_normal = vec4(0.0, 0.0, 0.0, 0.0);
                let dark_hover = (SLATE_700);
                let dark_selected = (SLATE_600);
                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);
                let base = mix(normal, hover_color, self.hover * (1.0 - self.selected));
                let color = mix(base, selected_color, self.selected);
                sdf.fill(color);
                return sdf.result;
            }
        }

        tab_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                instance selected: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    let light_normal = (SLATE_500);
                    let light_selected = (SLATE_900);
                    let dark_normal = (SLATE_400);
                    let dark_selected = (WHITE);
                    let normal = mix(light_normal, dark_normal, self.dark_mode);
                    let selected = mix(light_selected, dark_selected, self.dark_mode);
                    return mix(normal, selected, self.selected);
                }
            }
        }
    }

    // Reusable panel header style with dark mode support
    PanelHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (SLATE_800), self.dark_mode);
            }
        }
    }

    // Reusable vertical divider
    VerticalDivider = <View> {
        width: 1, height: Fill
        margin: {top: 4, bottom: 4}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
            }
        }
    }

    // MoFA FM Screen - adaptive horizontal layout with left content and right log panel
    pub MoFaFMScreen = {{MoFaFMScreen}} {
        width: Fill, height: Fill
        flow: Right
        spacing: 0
        padding: { left: 16, right: 16, top: 16, bottom: 16 }
        align: {y: 0.0}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Left column - main content area (adaptive width)
        left_column = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: (SECTION_SPACING)
            align: {y: 0.0}

            // System status bar (self-contained widget)
            mofa_hero = <MofaHero> {
                width: Fill
            }

            // Tab bar for Running/Settings
            tab_bar = <RoundedView> {
                width: Fill, height: Fit
                padding: 4
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    border_radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                        let bg = mix((SLATE_100), (SLATE_800), self.dark_mode);
                        sdf.fill(bg);
                        return sdf.result;
                    }
                }
                flow: Right
                spacing: 4

                running_tab = <TabButton> {
                    draw_bg: { selected: 1.0 }
                    tab_label = { text: "Running", draw_text: { selected: 1.0 } }
                }

                settings_tab = <TabButton> {
                    tab_label = { text: "Settings" }
                }
            }

            // Running tab content - visible by default
            running_tab_content = <View> {
                width: Fill, height: Fill
                flow: Down
                spacing: (SECTION_SPACING)

            // Participant status cards container
            participant_container = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                participant_bar = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: (SECTION_SPACING)

                    student1_panel = <ParticipantPanel> {
                        width: Fill, height: Fit
                        header = { name_label = { text: "Student 1" } }
                    }
                    student2_panel = <ParticipantPanel> {
                        width: Fill, height: Fit
                        header = { name_label = { text: "Student 2" } }
                    }
                    tutor_panel = <ParticipantPanel> {
                        width: Fill, height: Fit
                        header = { name_label = { text: "Tutor" } }
                    }
                }
            }

            // Chat window container (fills remaining space)
            chat_container = <View> {
                width: Fill, height: Fill
                flow: Down

                chat_section = <RoundedView> {
                    width: Fill, height: Fill
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }
                    flow: Down

                    // Chat header with copy button
                    chat_header = <PanelHeader> {
                        chat_title = <Label> {
                            text: "Chat History"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <Filler> {}
                        // Copy to clipboard button
                        copy_chat_btn = <View> {
                            width: 28, height: 24
                            cursor: Hand
                            show_bg: true
                            draw_bg: {
                                instance copied: 0.0
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let c = self.rect_size * 0.5;

                                    // Light theme: Green → Teal → Blue → Gray
                                    let gray_light = (BORDER);
                                    let blue_light = vec4(0.231, 0.510, 0.965, 1.0);   // #3b82f6
                                    let teal_light = vec4(0.078, 0.722, 0.651, 1.0);   // #14b8a6
                                    let green_light = vec4(0.133, 0.773, 0.373, 1.0);  // #22c55f

                                    // Dark theme: Bright Green → Cyan → Purple → Slate
                                    let gray_dark = vec4(0.334, 0.371, 0.451, 1.0);    // #555e73 (slate-600)
                                    let purple_dark = vec4(0.639, 0.380, 0.957, 1.0);  // #a361f4
                                    let cyan_dark = vec4(0.133, 0.831, 0.894, 1.0);    // #22d4e4
                                    let green_dark = vec4(0.290, 0.949, 0.424, 1.0);   // #4af26c

                                    // Select colors based on dark mode
                                    let gray = mix(gray_light, gray_dark, self.dark_mode);
                                    let c1 = mix(blue_light, purple_dark, self.dark_mode);
                                    let c2 = mix(teal_light, cyan_dark, self.dark_mode);
                                    let c3 = mix(green_light, green_dark, self.dark_mode);

                                    // Multi-stop gradient based on copied value
                                    let t = self.copied;
                                    let bg_color = mix(
                                        mix(mix(gray, c1, clamp(t * 3.0, 0.0, 1.0)),
                                            c2, clamp((t - 0.33) * 3.0, 0.0, 1.0)),
                                        c3, clamp((t - 0.66) * 3.0, 0.0, 1.0)
                                    );

                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                    sdf.fill(bg_color);

                                    // Icon color - white when active, gray otherwise
                                    let icon_base = mix((GRAY_600), vec4(0.580, 0.639, 0.722, 1.0), self.dark_mode);
                                    let icon_color = mix(icon_base, vec4(1.0, 1.0, 1.0, 1.0), smoothstep(0.0, 0.3, self.copied));

                                    // Clipboard icon - back rectangle
                                    sdf.box(c.x - 4.0, c.y - 2.0, 8.0, 9.0, 1.0);
                                    sdf.stroke(icon_color, 1.2);

                                    // Clipboard icon - front rectangle (overlapping)
                                    sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                    sdf.fill(bg_color);
                                    sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                    sdf.stroke(icon_color, 1.2);

                                    return sdf.result;
                                }
                            }
                        }
                    }

                    // Chat messages area (scrollable, fills space)
                    chat_scroll = <ScrollYView> {
                        width: Fill, height: Fill
                        flow: Down
                        scroll_bars: <ScrollBars> {
                            show_scroll_x: false
                            show_scroll_y: true
                        }

                        chat_content_wrapper = <View> {
                            width: Fill, height: Fit
                            padding: (PANEL_PADDING)
                            flow: Down

                            chat_content = <Markdown> {
                                width: Fill, height: Fit
                                font_size: 13.0
                                font_color: (TEXT_PRIMARY)
                                paragraph_spacing: 8

                                draw_normal: {
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                }
                                draw_bold: {
                                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                }
                            }
                        }
                    }
                }
            }

            // Audio control panel container - horizontal layout with individual containers
            audio_container = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: (SECTION_SPACING)

                // Mic level meter container
                mic_container = <RoundedView> {
                    width: Fit, height: Fit
                    padding: (PANEL_PADDING)
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }

                    mic_group = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 10
                        align: {y: 0.5}

                        mic_mute_btn = <View> {
                            width: Fit, height: Fit
                            flow: Overlay
                            cursor: Hand
                            padding: 4

                            mic_icon_on = <View> {
                                width: Fit, height: Fit
                                icon = <Icon> {
                                    draw_icon: {
                                        instance dark_mode: 0.0
                                        svg_file: dep("crate://self/resources/icons/mic.svg")
                                        fn get_color(self) -> vec4 {
                                            return mix((SLATE_500), (WHITE), self.dark_mode);
                                        }
                                    }
                                    icon_walk: {width: 20, height: 20}
                                }
                            }

                            mic_icon_off = <View> {
                                width: Fit, height: Fit
                                visible: false
                                <Icon> {
                                    draw_icon: {
                                        svg_file: dep("crate://self/resources/icons/mic-off.svg")
                                        fn get_color(self) -> vec4 { return (ACCENT_RED); }
                                    }
                                    icon_walk: {width: 20, height: 20}
                                }
                            }
                        }

                        mic_level_meter = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            spacing: 3
                            align: {y: 0.5}
                            padding: {top: 2, bottom: 2}

                            mic_led_1 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (GREEN_500), border_radius: 2.0 } }
                            mic_led_2 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (GREEN_500), border_radius: 2.0 } }
                            mic_led_3 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            mic_led_4 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            mic_led_5 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                        }
                    }
                }

                // AEC toggle container
                aec_container = <RoundedView> {
                    width: Fit, height: Fit
                    padding: (PANEL_PADDING)
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }

                    aec_group = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        aec_toggle_btn = <View> {
                            width: Fit, height: Fit
                            padding: 6
                            flow: Overlay
                            cursor: Hand
                            show_bg: true
                            draw_bg: {
                                instance enabled: 1.0  // 1.0=on, 0.0=off
                                // Blink animation now driven by shader time - no timer needed!
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                    let green = vec4(0.133, 0.773, 0.373, 1.0);
                                    let bright = vec4(0.2, 0.9, 0.5, 1.0);
                                    let gray = vec4(0.667, 0.686, 0.725, 1.0);
                                    // When enabled, pulse between green and bright green using shader time
                                    // sin(time * speed) creates smooth oscillation, step makes it blink
                                    let blink = step(0.0, sin(self.time * 2.0)) * self.enabled;
                                    let base = mix(gray, green, self.enabled);
                                    let col = mix(base, bright, blink * 0.5);
                                    sdf.fill(col);
                                    return sdf.result;
                                }
                            }
                            align: {x: 0.5, y: 0.5}

                            <Icon> {
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/icons/aec.svg")
                                    fn get_color(self) -> vec4 { return (WHITE); }
                                }
                                icon_walk: {width: 20, height: 20}
                            }
                        }
                    }
                }

                // Audio buffer indicator container
                buffer_container = <RoundedView> {
                    width: Fit, height: Fit
                    padding: (PANEL_PADDING)
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }

                    buffer_group = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        buffer_label = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_700), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                            text: "Buffer"
                        }

                        buffer_meter = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            spacing: 3
                            align: {y: 0.5}
                            padding: {top: 2, bottom: 2}

                            buffer_led_1 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            buffer_led_2 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            buffer_led_3 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            buffer_led_4 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                            buffer_led_5 = <RoundedView> { width: 8, height: 14, draw_bg: { color: (SLATE_200), border_radius: 2.0 } }
                        }

                        buffer_pct = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_500), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                            text: "0%"
                        }
                    }
                }

                // Device selectors container
                device_container = <RoundedView> {
                    width: Fill, height: Fit
                    padding: (PANEL_PADDING)
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }

                    device_selectors = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 6
                        align: {x: 0.5, y: 0.5}  // Center aligned

                        // Mic icon (green)
                        mic_icon = <Icon> {
                            draw_icon: {
                                svg_file: dep("crate://self/resources/icons/mic.svg")
                                fn get_color(self) -> vec4 {
                                    return vec4(0.133, 0.773, 0.373, 1.0);  // Green #22c55f
                                }
                            }
                            icon_walk: {width: 14, height: 14}
                        }

                        // Mic name label (same as buffer)
                        mic_name_label = <Label> {
                            width: Fit
                            text: "Microphone"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_700), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        // Audio device dropdown (≡ trigger)
                        audio_device_dropdown = <DropDown> {
                            width: 24, height: 24
                            margin: {left: 4, right: 0}
                            padding: 0
                            align: {x: 0.5, y: 0.5}
                            popup_menu_position: OnSelected
                            labels: ["≡"]
                            draw_bg: {
                                instance dark_mode: 0.0
                                border_radius: 4.0
                                border_size: 1.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                    let bg = mix((GRAY_100), (SLATE_700), self.dark_mode);
                                    let border = mix((BORDER), (SLATE_600), self.dark_mode);
                                    sdf.fill(bg);
                                    sdf.stroke(border, self.border_size);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                            values: []
                            selected_item: 0
                            popup_menu: {
                                width: 350
                                height: Fit
                                draw_bg: {
                                    instance dark_mode: 0.0
                                    border_size: 1.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                                        let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                                        let border = mix((BORDER), (SLATE_600), self.dark_mode);
                                        sdf.fill(bg);
                                        sdf.stroke(border, self.border_size);
                                        return sdf.result;
                                    }
                                }
                                menu_item: {
                                    width: Fill
                                    draw_bg: {
                                        instance dark_mode: 0.0
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                                            let base = mix((WHITE), (SLATE_800), self.dark_mode);
                                            let hover_color = mix((GRAY_100), (SLATE_700), self.dark_mode);
                                            sdf.fill(mix(base, hover_color, self.hover));
                                            return sdf.result;
                                        }
                                    }
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        fn get_color(self) -> vec4 {
                                            let light_base = mix((GRAY_700), (TEXT_PRIMARY), self.active);
                                            let dark_base = mix((SLATE_300), (TEXT_PRIMARY_DARK), self.active);
                                            let base = mix(light_base, dark_base, self.dark_mode);
                                            let light_hover = (TEXT_PRIMARY);
                                            let dark_hover = (TEXT_PRIMARY_DARK);
                                            let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                                            return mix(base, hover_color, self.hover);
                                        }
                                    }
                                }
                            }
                        }

                        // Speaker icon (green)
                        speaker_icon = <Icon> {
                            draw_icon: {
                                svg_file: dep("crate://self/resources/icons/speaker.svg")
                                fn get_color(self) -> vec4 {
                                    return vec4(0.133, 0.773, 0.373, 1.0);  // Green #22c55f
                                }
                            }
                            icon_walk: {width: 14, height: 14}
                        }

                        // Speaker name label (same as buffer)
                        speaker_name_label = <Label> {
                            width: Fit
                            text: "Speaker"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_700), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }
                }
            }

            // Prompt input area container
            prompt_container = <View> {
                width: Fill, height: Fit
                flow: Down

                prompt_section = <RoundedView> {
                    width: Fill, height: Fit
                    padding: (PANEL_PADDING)
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        fn get_color(self) -> vec4 {
                            return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                        }
                    }
                    flow: Down
                    spacing: 8

                    prompt_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12
                        align: {y: 0.5}

                        prompt_input = <TextInput> {
                            width: Fill, height: Fit
                            padding: {left: 12, right: 12, top: 10, bottom: 10}
                            empty_text: "Enter prompt to send..."
                            draw_bg: {
                                instance dark_mode: 0.0
                                border_radius: 4.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                    let bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                                    sdf.fill(bg);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                            draw_selection: {
                                color: (INDIGO_200)
                            }
                            draw_cursor: {
                                color: (ACCENT_BLUE)
                            }
                        }

                        button_group = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            spacing: 8

                            send_prompt_btn = <Button> {
                                width: Fit, height: 35
                                padding: {left: 16, right: 16}
                                text: "Send"

                                animator: {
                                    hover = {
                                        default: off,
                                        off = {
                                            from: {all: Forward {duration: 0.15}}
                                            apply: { draw_bg: {hover: 0.0} }
                                        }
                                        on = {
                                            from: {all: Forward {duration: 0.15}}
                                            apply: { draw_bg: {hover: 1.0} }
                                        }
                                    }
                                    pressed = {
                                        default: off,
                                        off = {
                                            from: {all: Forward {duration: 0.1}}
                                            apply: { draw_bg: {pressed: 0.0} }
                                        }
                                        on = {
                                            from: {all: Forward {duration: 0.1}}
                                            apply: { draw_bg: {pressed: 1.0} }
                                        }
                                    }
                                }

                                draw_text: {
                                    color: (WHITE)
                                    text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }
                                }
                                draw_bg: {
                                    instance hover: 0.0
                                    instance pressed: 0.0
                                    border_radius: 4.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        let color = mix(
                                            mix((ACCENT_BLUE), (BLUE_600), self.hover),
                                            (BLUE_700),
                                            self.pressed
                                        );
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                        sdf.fill(color);
                                        return sdf.result;
                                    }
                                }
                            }

                            reset_btn = <Button> {
                                width: Fit, height: 35
                                padding: {left: 16, right: 16}
                                text: "Reset"

                                animator: {
                                    hover = {
                                        default: off,
                                        off = {
                                            from: {all: Forward {duration: 0.15}}
                                            apply: { draw_bg: {hover: 0.0} }
                                        }
                                        on = {
                                            from: {all: Forward {duration: 0.15}}
                                            apply: { draw_bg: {hover: 1.0} }
                                        }
                                    }
                                    pressed = {
                                        default: off,
                                        off = {
                                            from: {all: Forward {duration: 0.1}}
                                            apply: { draw_bg: {pressed: 0.0} }
                                        }
                                        on = {
                                            from: {all: Forward {duration: 0.1}}
                                            apply: { draw_bg: {pressed: 1.0} }
                                        }
                                    }
                                }

                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((GRAY_700), (SLATE_300), self.dark_mode);
                                    }
                                }
                                draw_bg: {
                                    instance hover: 0.0
                                    instance pressed: 0.0
                                    instance dark_mode: 0.0
                                    border_radius: 4.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                        let base = mix((HOVER_BG), (SLATE_600), self.dark_mode);
                                        let hover_color = mix((SLATE_200), (SLATE_500), self.dark_mode);
                                        let pressed_color = mix((SLATE_300), (SLATE_400), self.dark_mode);
                                        let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                                        sdf.fill(color);
                                        return sdf.result;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            } // end running_tab_content

            // Settings tab content - hidden by default
            settings_tab_content = <View> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                spacing: (SECTION_SPACING)

                // Settings panel
                settings_panel = <RoundedView> {
                    width: Fill, height: Fill
                    padding: 20
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: (PANEL_RADIUS)
                        border_size: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let bg = mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                            let border = mix((BORDER), (SLATE_600), self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, self.border_size);
                            return sdf.result;
                        }
                    }
                    flow: Down
                    spacing: 20

                    // Settings header
                    settings_header = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 4

                        settings_title = <Label> {
                            text: "MoFA FM Settings"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        settings_subtitle = <Label> {
                            text: "Configure voice chat settings and participant options"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Divider
                    <View> {
                        width: Fill, height: 1
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                            }
                        }
                    }

                    // Settings content - scrollable
                    settings_scroll = <ScrollYView> {
                        width: Fill, height: Fill
                        flow: Down
                        scroll_bars: <ScrollBars> {
                            show_scroll_x: false
                            show_scroll_y: true
                        }

                        settings_content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 24
                            padding: {right: 12}

                            // Dataflow Configuration Section
                            dataflow_section = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 12

                                dataflow_section_title = <Label> {
                                    text: "Dataflow Configuration"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                // Dataflow path row
                                dataflow_path_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    dataflow_path_label = <Label> {
                                        width: 120
                                        text: "Dataflow Path"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    dataflow_path_value = <Label> {
                                        width: Fill
                                        text: "apps/mofa-fm/dataflow/voice-chat.yml"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((GRAY_600), (SLATE_400), self.dark_mode);
                                            }
                                        }
                                    }
                                }
                            }

                            // Role Configuration Section
                            role_section = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 16

                                role_section_title = <Label> {
                                    text: "Role Configuration"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                // Student 1 Configuration
                                student1_config = <RoundedView> {
                                    width: Fill, height: Fit
                                    padding: 16
                                    show_bg: true
                                    draw_bg: {
                                        instance dark_mode: 0.0
                                        border_radius: 8.0
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                            let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                                            sdf.fill(bg);
                                            return sdf.result;
                                        }
                                    }
                                    flow: Down
                                    spacing: 12

                                    // Header row with name and status
                                    student1_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 8
                                        align: {y: 0.5}

                                        student1_name = <Label> {
                                            text: "Student 1 (大牛)"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        student1_status = <Label> {
                                            text: "Active"
                                            draw_text: {
                                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (GREEN_500);
                                                }
                                            }
                                        }
                                    }

                                    // Model selection row
                                    student1_model_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 12
                                        align: {y: 0.5}

                                        student1_model_label = <Label> {
                                            width: 100
                                            text: "Model"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        student1_model_dropdown = <DropDown> {
                                            width: 200, height: Fit
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                            popup_menu: {
                                                menu_item: {
                                                    indent_width: 10.0
                                                    padding: {left: 15, top: 8, bottom: 8, right: 15}
                                                }
                                            }
                                            labels: ["gpt-4o", "gpt-4o-mini", "deepseek-chat"]
                                            selected_item: 0
                                        }
                                    }

                                    // System Prompt Header with maximize button
                                    student1_prompt_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {y: 0.5}

                                        student1_prompt_label = <Label> {
                                            text: "System Prompt"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        student1_maximize_btn = <View> {
                                            width: 20, height: 20
                                            cursor: Hand
                                            show_bg: true
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance maximized: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    let color = mix(vec4(0.4, 0.45, 0.5, 1.0), vec4(0.7, 0.75, 0.8, 1.0), self.dark_mode);
                                                    let cx = self.rect_size.x * 0.5;
                                                    let cy = self.rect_size.y * 0.5;
                                                    if self.maximized < 0.5 {
                                                        sdf.move_to(cx - 3.0, cy - 3.0); sdf.line_to(cx - 6.0, cy - 6.0);
                                                        sdf.move_to(cx + 3.0, cy - 3.0); sdf.line_to(cx + 6.0, cy - 6.0);
                                                        sdf.move_to(cx - 3.0, cy + 3.0); sdf.line_to(cx - 6.0, cy + 6.0);
                                                        sdf.move_to(cx + 3.0, cy + 3.0); sdf.line_to(cx + 6.0, cy + 6.0);
                                                    } else {
                                                        sdf.move_to(cx - 6.0, cy - 6.0); sdf.line_to(cx - 3.0, cy - 3.0);
                                                        sdf.move_to(cx + 6.0, cy - 6.0); sdf.line_to(cx + 3.0, cy - 3.0);
                                                        sdf.move_to(cx - 6.0, cy + 6.0); sdf.line_to(cx - 3.0, cy + 3.0);
                                                        sdf.move_to(cx + 6.0, cy + 6.0); sdf.line_to(cx + 3.0, cy + 3.0);
                                                    }
                                                    sdf.stroke(color, 1.2);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }

                                    student1_prompt_container = <RoundedView> {
                                        width: Fill, height: 120
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            border_radius: 4.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                                let bg = mix((WHITE), (SLATE_600), self.dark_mode);
                                                let border = mix((SLATE_300), (SLATE_500), self.dark_mode);
                                                sdf.fill(bg);
                                                sdf.stroke(border, 1.0);
                                                return sdf.result;
                                            }
                                        }

                                        student1_prompt_scroll = <ScrollYView> {
                                            width: Fill, height: Fill
                                            scroll_bars: <ScrollBars> {
                                                show_scroll_x: false
                                                show_scroll_y: true
                                                scroll_bar_y: {
                                                    bar_size: 6.0
                                                    smoothing: 0.15
                                                }
                                            }

                                            student1_prompt_wrapper = <View> {
                                                width: Fill, height: Fit
                                                padding: 8

                                                student1_prompt_input = <TextInput> {
                                                    width: Fill, height: Fit
                                                    draw_bg: {
                                                        fn pixel(self) -> vec4 {
                                                            return vec4(0.0, 0.0, 0.0, 0.0);
                                                        }
                                                    }
                                                    draw_text: {
                                                        instance dark_mode: 0.0
                                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                        fn get_color(self) -> vec4 {
                                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                        }
                                                    }
                                                    draw_selection: {
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.0);
                                                            sdf.fill(vec4(0.26, 0.52, 0.96, 0.4));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    draw_cursor: {
                                                        instance focus: 0.0
                                                        instance blink: 0.0
                                                        instance dark_mode: 0.0
                                                        uniform border_radius: 0.5
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                                                            let cursor_color = mix(vec4(0.1, 0.1, 0.12, 1.0), vec4(0.9, 0.9, 0.95, 1.0), self.dark_mode);
                                                            sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), cursor_color, (1.0 - self.blink) * self.focus));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    animator: {
                                                        blink = {
                                                            default: off
                                                            off = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 0.0} }
                                                            }
                                                            on = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 1.0} }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Save button row
                                    student1_save_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {x: 1.0, y: 0.5}

                                        student1_save_btn = <Button> {
                                            width: Fit, height: Fit
                                            padding: {left: 16, right: 16, top: 8, bottom: 8}
                                            text: "Save"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (WHITE);
                                                }
                                            }
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance hover: 0.0
                                                instance pressed: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                                    let base = mix((ACCENT_BLUE), (BLUE_600), self.dark_mode);
                                                    let hover_color = mix((BLUE_600), (BLUE_500), self.dark_mode);
                                                    let pressed_color = mix((BLUE_700), (BLUE_400), self.dark_mode);
                                                    let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                                                    sdf.fill(color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }

                                // Student 2 Configuration
                                student2_config = <RoundedView> {
                                    width: Fill, height: Fit
                                    padding: 16
                                    show_bg: true
                                    draw_bg: {
                                        instance dark_mode: 0.0
                                        border_radius: 8.0
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                            let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                                            sdf.fill(bg);
                                            return sdf.result;
                                        }
                                    }
                                    flow: Down
                                    spacing: 12

                                    student2_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 8
                                        align: {y: 0.5}

                                        student2_name = <Label> {
                                            text: "Student 2 (亦菲)"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        student2_status = <Label> {
                                            text: "Active"
                                            draw_text: {
                                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (GREEN_500);
                                                }
                                            }
                                        }
                                    }

                                    student2_model_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 12
                                        align: {y: 0.5}

                                        student2_model_label = <Label> {
                                            width: 100
                                            text: "Model"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        student2_model_dropdown = <DropDown> {
                                            width: 200, height: Fit
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                            popup_menu: {
                                                menu_item: {
                                                    indent_width: 10.0
                                                    padding: {left: 15, top: 8, bottom: 8, right: 15}
                                                }
                                            }
                                            labels: ["gpt-4o", "gpt-4o-mini", "deepseek-chat"]
                                            selected_item: 0
                                        }
                                    }

                                    // System Prompt Header with maximize button
                                    student2_prompt_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {y: 0.5}

                                        student2_prompt_label = <Label> {
                                            text: "System Prompt"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        student2_maximize_btn = <View> {
                                            width: 20, height: 20
                                            cursor: Hand
                                            show_bg: true
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance maximized: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    let color = mix(vec4(0.4, 0.45, 0.5, 1.0), vec4(0.7, 0.75, 0.8, 1.0), self.dark_mode);
                                                    let cx = self.rect_size.x * 0.5;
                                                    let cy = self.rect_size.y * 0.5;
                                                    if self.maximized < 0.5 {
                                                        sdf.move_to(cx - 3.0, cy - 3.0); sdf.line_to(cx - 6.0, cy - 6.0);
                                                        sdf.move_to(cx + 3.0, cy - 3.0); sdf.line_to(cx + 6.0, cy - 6.0);
                                                        sdf.move_to(cx - 3.0, cy + 3.0); sdf.line_to(cx - 6.0, cy + 6.0);
                                                        sdf.move_to(cx + 3.0, cy + 3.0); sdf.line_to(cx + 6.0, cy + 6.0);
                                                    } else {
                                                        sdf.move_to(cx - 6.0, cy - 6.0); sdf.line_to(cx - 3.0, cy - 3.0);
                                                        sdf.move_to(cx + 6.0, cy - 6.0); sdf.line_to(cx + 3.0, cy - 3.0);
                                                        sdf.move_to(cx - 6.0, cy + 6.0); sdf.line_to(cx - 3.0, cy + 3.0);
                                                        sdf.move_to(cx + 6.0, cy + 6.0); sdf.line_to(cx + 3.0, cy + 3.0);
                                                    }
                                                    sdf.stroke(color, 1.2);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }

                                    student2_prompt_container = <RoundedView> {
                                        width: Fill, height: 120
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            border_radius: 4.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                                let bg = mix((WHITE), (SLATE_600), self.dark_mode);
                                                let border = mix((SLATE_300), (SLATE_500), self.dark_mode);
                                                sdf.fill(bg);
                                                sdf.stroke(border, 1.0);
                                                return sdf.result;
                                            }
                                        }

                                        student2_prompt_scroll = <ScrollYView> {
                                            width: Fill, height: Fill
                                            scroll_bars: <ScrollBars> {
                                                show_scroll_x: false
                                                show_scroll_y: true
                                                scroll_bar_y: {
                                                    bar_size: 6.0
                                                    smoothing: 0.15
                                                }
                                            }

                                            student2_prompt_wrapper = <View> {
                                                width: Fill, height: Fit
                                                padding: 8

                                                student2_prompt_input = <TextInput> {
                                                    width: Fill, height: Fit
                                                    draw_bg: {
                                                        fn pixel(self) -> vec4 {
                                                            return vec4(0.0, 0.0, 0.0, 0.0);
                                                        }
                                                    }
                                                    draw_text: {
                                                        instance dark_mode: 0.0
                                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                        fn get_color(self) -> vec4 {
                                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                        }
                                                    }
                                                    draw_selection: {
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.0);
                                                            sdf.fill(vec4(0.26, 0.52, 0.96, 0.4));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    draw_cursor: {
                                                        instance focus: 0.0
                                                        instance blink: 0.0
                                                        instance dark_mode: 0.0
                                                        uniform border_radius: 0.5
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                                                            let cursor_color = mix(vec4(0.1, 0.1, 0.12, 1.0), vec4(0.9, 0.9, 0.95, 1.0), self.dark_mode);
                                                            sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), cursor_color, (1.0 - self.blink) * self.focus));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    animator: {
                                                        blink = {
                                                            default: off
                                                            off = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 0.0} }
                                                            }
                                                            on = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 1.0} }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    student2_save_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {x: 1.0, y: 0.5}

                                        student2_save_btn = <Button> {
                                            width: Fit, height: Fit
                                            padding: {left: 16, right: 16, top: 8, bottom: 8}
                                            text: "Save"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (WHITE);
                                                }
                                            }
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance hover: 0.0
                                                instance pressed: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                                    let base = mix((ACCENT_BLUE), (BLUE_600), self.dark_mode);
                                                    let hover_color = mix((BLUE_600), (BLUE_500), self.dark_mode);
                                                    let pressed_color = mix((BLUE_700), (BLUE_400), self.dark_mode);
                                                    let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                                                    sdf.fill(color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }

                                // Tutor Configuration
                                tutor_config = <RoundedView> {
                                    width: Fill, height: Fit
                                    padding: 16
                                    show_bg: true
                                    draw_bg: {
                                        instance dark_mode: 0.0
                                        border_radius: 8.0
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                            let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                                            sdf.fill(bg);
                                            return sdf.result;
                                        }
                                    }
                                    flow: Down
                                    spacing: 12

                                    tutor_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 8
                                        align: {y: 0.5}

                                        tutor_name = <Label> {
                                            text: "Tutor (孙老师)"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        tutor_status = <Label> {
                                            text: "Active"
                                            draw_text: {
                                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (GREEN_500);
                                                }
                                            }
                                        }
                                    }

                                    tutor_model_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 12
                                        align: {y: 0.5}

                                        tutor_model_label = <Label> {
                                            width: 100
                                            text: "Model"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        tutor_model_dropdown = <DropDown> {
                                            width: 200, height: Fit
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                            popup_menu: {
                                                menu_item: {
                                                    indent_width: 10.0
                                                    padding: {left: 15, top: 8, bottom: 8, right: 15}
                                                }
                                            }
                                            labels: ["gpt-4o", "gpt-4o-mini", "deepseek-chat"]
                                            selected_item: 0
                                        }
                                    }

                                    // System Prompt Header with maximize button
                                    tutor_prompt_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {y: 0.5}

                                        tutor_prompt_label = <Label> {
                                            text: "System Prompt"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        tutor_maximize_btn = <View> {
                                            width: 20, height: 20
                                            cursor: Hand
                                            show_bg: true
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance maximized: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    let color = mix(vec4(0.4, 0.45, 0.5, 1.0), vec4(0.7, 0.75, 0.8, 1.0), self.dark_mode);
                                                    let cx = self.rect_size.x * 0.5;
                                                    let cy = self.rect_size.y * 0.5;
                                                    if self.maximized < 0.5 {
                                                        sdf.move_to(cx - 3.0, cy - 3.0); sdf.line_to(cx - 6.0, cy - 6.0);
                                                        sdf.move_to(cx + 3.0, cy - 3.0); sdf.line_to(cx + 6.0, cy - 6.0);
                                                        sdf.move_to(cx - 3.0, cy + 3.0); sdf.line_to(cx - 6.0, cy + 6.0);
                                                        sdf.move_to(cx + 3.0, cy + 3.0); sdf.line_to(cx + 6.0, cy + 6.0);
                                                    } else {
                                                        sdf.move_to(cx - 6.0, cy - 6.0); sdf.line_to(cx - 3.0, cy - 3.0);
                                                        sdf.move_to(cx + 6.0, cy - 6.0); sdf.line_to(cx + 3.0, cy - 3.0);
                                                        sdf.move_to(cx - 6.0, cy + 6.0); sdf.line_to(cx - 3.0, cy + 3.0);
                                                        sdf.move_to(cx + 6.0, cy + 6.0); sdf.line_to(cx + 3.0, cy + 3.0);
                                                    }
                                                    sdf.stroke(color, 1.2);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }

                                    tutor_prompt_container = <RoundedView> {
                                        width: Fill, height: 120
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            border_radius: 4.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                                let bg = mix((WHITE), (SLATE_600), self.dark_mode);
                                                let border = mix((SLATE_300), (SLATE_500), self.dark_mode);
                                                sdf.fill(bg);
                                                sdf.stroke(border, 1.0);
                                                return sdf.result;
                                            }
                                        }

                                        tutor_prompt_scroll = <ScrollYView> {
                                            width: Fill, height: Fill
                                            scroll_bars: <ScrollBars> {
                                                show_scroll_x: false
                                                show_scroll_y: true
                                                scroll_bar_y: {
                                                    bar_size: 6.0
                                                    smoothing: 0.15
                                                }
                                            }

                                            tutor_prompt_wrapper = <View> {
                                                width: Fill, height: Fit
                                                padding: 8

                                                tutor_prompt_input = <TextInput> {
                                                    width: Fill, height: Fit
                                                    draw_bg: {
                                                        fn pixel(self) -> vec4 {
                                                            return vec4(0.0, 0.0, 0.0, 0.0);
                                                        }
                                                    }
                                                    draw_text: {
                                                        instance dark_mode: 0.0
                                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                        fn get_color(self) -> vec4 {
                                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                        }
                                                    }
                                                    draw_selection: {
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.0);
                                                            sdf.fill(vec4(0.26, 0.52, 0.96, 0.4));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    draw_cursor: {
                                                        instance focus: 0.0
                                                        instance blink: 0.0
                                                        instance dark_mode: 0.0
                                                        uniform border_radius: 0.5
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                                                            let cursor_color = mix(vec4(0.1, 0.1, 0.12, 1.0), vec4(0.9, 0.9, 0.95, 1.0), self.dark_mode);
                                                            sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), cursor_color, (1.0 - self.blink) * self.focus));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    animator: {
                                                        blink = {
                                                            default: off
                                                            off = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 0.0} }
                                                            }
                                                            on = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 1.0} }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    tutor_save_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {x: 1.0, y: 0.5}

                                        tutor_save_btn = <Button> {
                                            width: Fit, height: Fit
                                            padding: {left: 16, right: 16, top: 8, bottom: 8}
                                            text: "Save"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (WHITE);
                                                }
                                            }
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance hover: 0.0
                                                instance pressed: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                                    let base = mix((ACCENT_BLUE), (BLUE_600), self.dark_mode);
                                                    let hover_color = mix((BLUE_600), (BLUE_500), self.dark_mode);
                                                    let pressed_color = mix((BLUE_700), (BLUE_400), self.dark_mode);
                                                    let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                                                    sdf.fill(color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }

                                // Shared System Context Section
                                context_section = <RoundedView> {
                                    width: Fill, height: Fit
                                    padding: 16
                                    show_bg: true
                                    draw_bg: {
                                        instance dark_mode: 0.0
                                        border_radius: 8.0
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                            let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                                            sdf.fill(bg);
                                            return sdf.result;
                                        }
                                    }
                                    flow: Down
                                    spacing: 12

                                    context_header = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 8
                                        align: {y: 0.5}

                                        context_title = <Label> {
                                            text: "Shared System Context (study-context.md)"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                                fn get_color(self) -> vec4 {
                                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                }
                                            }
                                        }

                                        <View> { width: Fill, height: 1 }

                                        context_status = <Label> {
                                            text: "Loaded"
                                            draw_text: {
                                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (GREEN_500);
                                                }
                                            }
                                        }

                                        context_maximize_btn = <View> {
                                            width: 24, height: 24
                                            cursor: Hand
                                            show_bg: true
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance maximized: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    let color = mix(vec4(0.4, 0.45, 0.5, 1.0), vec4(0.7, 0.75, 0.8, 1.0), self.dark_mode);
                                                    let cx = self.rect_size.x * 0.5;
                                                    let cy = self.rect_size.y * 0.5;

                                                    if self.maximized < 0.5 {
                                                        // Expand icon (arrows pointing outward)
                                                        sdf.move_to(cx - 4.0, cy - 4.0);
                                                        sdf.line_to(cx - 8.0, cy - 8.0);
                                                        sdf.move_to(cx + 4.0, cy - 4.0);
                                                        sdf.line_to(cx + 8.0, cy - 8.0);
                                                        sdf.move_to(cx - 4.0, cy + 4.0);
                                                        sdf.line_to(cx - 8.0, cy + 8.0);
                                                        sdf.move_to(cx + 4.0, cy + 4.0);
                                                        sdf.line_to(cx + 8.0, cy + 8.0);
                                                    } else {
                                                        // Collapse icon (arrows pointing inward)
                                                        sdf.move_to(cx - 8.0, cy - 8.0);
                                                        sdf.line_to(cx - 4.0, cy - 4.0);
                                                        sdf.move_to(cx + 8.0, cy - 8.0);
                                                        sdf.line_to(cx + 4.0, cy - 4.0);
                                                        sdf.move_to(cx - 8.0, cy + 8.0);
                                                        sdf.line_to(cx - 4.0, cy + 4.0);
                                                        sdf.move_to(cx + 8.0, cy + 8.0);
                                                        sdf.line_to(cx + 4.0, cy + 4.0);
                                                    }
                                                    sdf.stroke(color, 1.5);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }

                                    context_input_container = <RoundedView> {
                                        width: Fill, height: 200
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            border_radius: 4.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                                let bg = mix((WHITE), (SLATE_600), self.dark_mode);
                                                let border = mix((SLATE_300), (SLATE_500), self.dark_mode);
                                                sdf.fill(bg);
                                                sdf.stroke(border, 1.0);
                                                return sdf.result;
                                            }
                                        }

                                        context_input_scroll = <ScrollYView> {
                                            width: Fill, height: Fill
                                            scroll_bars: <ScrollBars> {
                                                show_scroll_x: false
                                                show_scroll_y: true
                                                scroll_bar_y: {
                                                    bar_size: 6.0
                                                    smoothing: 0.15
                                                }
                                            }

                                            context_input_wrapper = <View> {
                                                width: Fill, height: Fit
                                                padding: 8

                                                context_input = <TextInput> {
                                                    width: Fill, height: Fit
                                                    draw_bg: {
                                                        fn pixel(self) -> vec4 {
                                                            return vec4(0.0, 0.0, 0.0, 0.0);
                                                        }
                                                    }
                                                    draw_text: {
                                                        instance dark_mode: 0.0
                                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                                        fn get_color(self) -> vec4 {
                                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                                        }
                                                    }
                                                    draw_selection: {
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.0);
                                                            sdf.fill(vec4(0.26, 0.52, 0.96, 0.4));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    draw_cursor: {
                                                        instance focus: 0.0
                                                        instance blink: 0.0
                                                        instance dark_mode: 0.0
                                                        uniform border_radius: 0.5
                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                                                            let cursor_color = mix(vec4(0.1, 0.1, 0.12, 1.0), vec4(0.9, 0.9, 0.95, 1.0), self.dark_mode);
                                                            sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), cursor_color, (1.0 - self.blink) * self.focus));
                                                            return sdf.result;
                                                        }
                                                    }
                                                    animator: {
                                                        blink = {
                                                            default: off
                                                            off = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 0.0} }
                                                            }
                                                            on = {
                                                                from: {all: Forward {duration: 0.5}}
                                                                apply: { draw_cursor: {blink: 1.0} }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    context_save_row = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        align: {x: 1.0, y: 0.5}

                                        context_save_btn = <Button> {
                                            width: Fit, height: Fit
                                            padding: {left: 16, right: 16, top: 8, bottom: 8}
                                            text: "Save Context"
                                            draw_text: {
                                                instance dark_mode: 0.0
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                fn get_color(self) -> vec4 {
                                                    return (WHITE);
                                                }
                                            }
                                            draw_bg: {
                                                instance dark_mode: 0.0
                                                instance hover: 0.0
                                                instance pressed: 0.0
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                                    let base = mix((ACCENT_BLUE), (BLUE_600), self.dark_mode);
                                                    let hover_color = mix((BLUE_600), (BLUE_500), self.dark_mode);
                                                    let pressed_color = mix((BLUE_700), (BLUE_400), self.dark_mode);
                                                    let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                                                    sdf.fill(color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Audio Settings Section
                            audio_section = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 12

                                audio_section_title = <Label> {
                                    text: "Audio Settings"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                // Sample rate row
                                sample_rate_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    sample_rate_label = <Label> {
                                        width: 120
                                        text: "Sample Rate"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    sample_rate_value = <Label> {
                                        text: "32000 Hz"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((GRAY_600), (SLATE_400), self.dark_mode);
                                            }
                                        }
                                    }
                                }

                                // Buffer size row
                                buffer_size_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    buffer_size_label = <Label> {
                                        width: 120
                                        text: "Buffer Size"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    buffer_size_value = <Label> {
                                        text: "5 seconds"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((GRAY_600), (SLATE_400), self.dark_mode);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Splitter - draggable handle with padding
        splitter = <View> {
            width: 16, height: Fill
            margin: { left: 8, right: 8 }
            align: {y: 0.0}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    // Draw thin line in center
                    sdf.rect(7.0, 16.0, 2.0, self.rect_size.y - 32.0);
                    let color = mix((SLATE_300), (SLATE_600), self.dark_mode);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
            cursor: ColResize
        }

        // System Log panel - adaptive width, top-aligned
        log_section = <View> {
            width: 320, height: Fill
            flow: Right
            align: {y: 0.0}

            // Toggle button column
            toggle_column = <View> {
                width: Fit, height: Fill
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix((SLATE_50), (SLATE_800), self.dark_mode);
                    }
                }
                align: {x: 0.5, y: 0.0}
                padding: {left: 4, right: 4, top: 8}

                toggle_log_btn = <Button> {
                    width: Fit, height: Fit
                    padding: {left: 8, right: 8, top: 6, bottom: 6}
                    text: ">"

                    animator: {
                        hover = {
                            default: off,
                            off = {
                                from: {all: Forward {duration: 0.15}}
                                apply: { draw_bg: {hover: 0.0} }
                            }
                            on = {
                                from: {all: Forward {duration: 0.15}}
                                apply: { draw_bg: {hover: 1.0} }
                            }
                        }
                        pressed = {
                            default: off,
                            off = {
                                from: {all: Forward {duration: 0.1}}
                                apply: { draw_bg: {pressed: 0.0} }
                            }
                            on = {
                                from: {all: Forward {duration: 0.1}}
                                apply: { draw_bg: {pressed: 1.0} }
                            }
                        }
                    }

                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((SLATE_500), (SLATE_400), self.dark_mode);
                        }
                    }
                    draw_bg: {
                        instance hover: 0.0
                        instance pressed: 0.0
                        instance dark_mode: 0.0
                        border_radius: 4.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                            let base = mix((SLATE_200), (SLATE_600), self.dark_mode);
                            let hover_color = mix((SLATE_300), (SLATE_500), self.dark_mode);
                            let pressed_color = mix((SLATE_400), (SLATE_400), self.dark_mode);
                            let color = mix(mix(base, hover_color, self.hover), pressed_color, self.pressed);
                            sdf.fill(color);
                            return sdf.result;
                        }
                    }
                }
            }

            // Log content panel
            log_content_column = <RoundedView> {
                width: Fill, height: Fill
                draw_bg: {
                    instance dark_mode: 0.0
                    border_radius: (PANEL_RADIUS)
                    fn get_color(self) -> vec4 {
                        return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                    }
                }
                flow: Down

                log_header = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((SLATE_50), (SLATE_800), self.dark_mode);
                        }
                    }

                    // Title row
                    log_title_row = <View> {
                        width: Fill, height: Fit
                        padding: {left: 12, right: 12, top: 10, bottom: 6}
                        log_title_label = <Label> {
                            text: "System Log"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Filter row
                    log_filter_row = <View> {
                        width: Fill, height: 32
                        flow: Right
                        align: {y: 0.5}
                        padding: {left: 8, right: 8, bottom: 6}
                        spacing: 6

                        // Level filter dropdown
                        level_filter = <DropDown> {
                            width: 70, height: 24
                            popup_menu_position: BelowInput
                            draw_bg: {
                                color: (HOVER_BG)
                                border_color: (SLATE_200)
                                border_radius: 2.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    // Background
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                                    sdf.fill((HOVER_BG));
                                    // Down arrow on right side
                                    let ax = self.rect_size.x - 12.0;
                                    let ay = self.rect_size.y * 0.5 - 2.0;
                                    sdf.move_to(ax - 3.0, ay);
                                    sdf.line_to(ax, ay + 4.0);
                                    sdf.line_to(ax + 3.0, ay);
                                    sdf.stroke((TEXT_PRIMARY), 1.5);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return (TEXT_PRIMARY);
                                }
                            }
                            popup_menu: {
                                draw_bg: {
                                    color: (WHITE)
                                    border_color: (BORDER)
                                    border_size: 1.0
                                    border_radius: 2.0
                                }
                                menu_item: {
                                    draw_bg: {
                                        color: (WHITE)
                                        color_hover: (GRAY_100)
                                    }
                                    draw_text: {
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                mix((GRAY_700), (TEXT_PRIMARY), self.active),
                                                (TEXT_PRIMARY),
                                                self.hover
                                            );
                                        }
                                    }
                                }
                            }
                            labels: ["ALL", "DEBUG", "INFO", "WARN", "ERROR"]
                            values: [ALL, DEBUG, INFO, WARN, ERROR]
                        }

                        // Node filter dropdown
                        node_filter = <DropDown> {
                            width: 85, height: 24
                            popup_menu_position: BelowInput
                            draw_bg: {
                                color: (HOVER_BG)
                                border_color: (SLATE_200)
                                border_radius: 2.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    // Background
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                                    sdf.fill((HOVER_BG));
                                    // Down arrow on right side
                                    let ax = self.rect_size.x - 12.0;
                                    let ay = self.rect_size.y * 0.5 - 2.0;
                                    sdf.move_to(ax - 3.0, ay);
                                    sdf.line_to(ax, ay + 4.0);
                                    sdf.line_to(ax + 3.0, ay);
                                    sdf.stroke((TEXT_PRIMARY), 1.5);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return (TEXT_PRIMARY);
                                }
                            }
                            popup_menu: {
                                draw_bg: {
                                    color: (WHITE)
                                    border_color: (BORDER)
                                    border_size: 1.0
                                    border_radius: 2.0
                                }
                                menu_item: {
                                    draw_bg: {
                                        color: (WHITE)
                                        color_hover: (GRAY_100)
                                    }
                                    draw_text: {
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                mix((GRAY_700), (TEXT_PRIMARY), self.active),
                                                (TEXT_PRIMARY),
                                                self.hover
                                            );
                                        }
                                    }
                                }
                            }
                            labels: ["All Nodes", "ASR", "TTS", "LLM", "Bridge", "Monitor", "App"]
                            values: [ALL, ASR, TTS, LLM, BRIDGE, MONITOR, APP]
                        }

                        // Search icon
                        search_icon = <View> {
                            width: 20, height: 24
                            align: {x: 0.5, y: 0.5}
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let c = self.rect_size * 0.5;
                                    // Magnifying glass circle
                                    sdf.circle(c.x - 2.0, c.y - 2.0, 5.0);
                                    sdf.stroke((GRAY_500), 1.5);
                                    // Handle
                                    sdf.move_to(c.x + 1.5, c.y + 1.5);
                                    sdf.line_to(c.x + 6.0, c.y + 6.0);
                                    sdf.stroke((GRAY_500), 1.5);
                                    return sdf.result;
                                }
                            }
                        }

                        // Search field
                        log_search = <TextInput> {
                            width: Fill, height: 24
                            empty_text: "Search..."
                            draw_bg: {
                                instance dark_mode: 0.0
                                border_radius: 2.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                    let bg = mix((WHITE), (SLATE_700), self.dark_mode);
                                    sdf.fill(bg);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                            draw_selection: {
                                color: (INDIGO_200)
                            }
                            draw_cursor: {
                                color: (ACCENT_BLUE)
                            }
                        }

                        // Copy to clipboard button
                        copy_log_btn = <View> {
                            width: 28, height: 24
                            cursor: Hand
                            show_bg: true
                            draw_bg: {
                                instance copied: 0.0
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let c = self.rect_size * 0.5;

                                    // Light theme: Green → Teal → Blue → Gray
                                    let gray_light = (BORDER);
                                    let blue_light = vec4(0.231, 0.510, 0.965, 1.0);   // #3b82f6
                                    let teal_light = vec4(0.078, 0.722, 0.651, 1.0);   // #14b8a6
                                    let green_light = vec4(0.133, 0.773, 0.373, 1.0);  // #22c55f

                                    // Dark theme: Bright Green → Cyan → Purple → Slate
                                    let gray_dark = vec4(0.334, 0.371, 0.451, 1.0);    // #555e73 (slate-600)
                                    let purple_dark = vec4(0.639, 0.380, 0.957, 1.0);  // #a361f4
                                    let cyan_dark = vec4(0.133, 0.831, 0.894, 1.0);    // #22d4e4
                                    let green_dark = vec4(0.290, 0.949, 0.424, 1.0);   // #4af26c

                                    // Select colors based on dark mode
                                    let gray = mix(gray_light, gray_dark, self.dark_mode);
                                    let c1 = mix(blue_light, purple_dark, self.dark_mode);
                                    let c2 = mix(teal_light, cyan_dark, self.dark_mode);
                                    let c3 = mix(green_light, green_dark, self.dark_mode);

                                    // Multi-stop gradient based on copied value
                                    let t = self.copied;
                                    let bg_color = mix(
                                        mix(mix(gray, c1, clamp(t * 3.0, 0.0, 1.0)),
                                            c2, clamp((t - 0.33) * 3.0, 0.0, 1.0)),
                                        c3, clamp((t - 0.66) * 3.0, 0.0, 1.0)
                                    );

                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                    sdf.fill(bg_color);

                                    // Icon color - white when active, gray otherwise
                                    let icon_base = mix((GRAY_600), vec4(0.580, 0.639, 0.722, 1.0), self.dark_mode);
                                    let icon_color = mix(icon_base, vec4(1.0, 1.0, 1.0, 1.0), smoothstep(0.0, 0.3, self.copied));

                                    // Clipboard icon - back rectangle
                                    sdf.box(c.x - 4.0, c.y - 2.0, 8.0, 9.0, 1.0);
                                    sdf.stroke(icon_color, 1.2);

                                    // Clipboard icon - front rectangle (overlapping)
                                    sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                    sdf.fill(bg_color);
                                    sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                    sdf.stroke(icon_color, 1.2);

                                    return sdf.result;
                                }
                            }
                        }
                    }
                }

                log_scroll = <ScrollYView> {
                    width: Fill, height: Fill
                    flow: Down
                    scroll_bars: <ScrollBars> {
                        show_scroll_x: false
                        show_scroll_y: true
                    }

                    log_content_wrapper = <View> {
                        width: Fill, height: Fit
                        padding: { left: 12, right: 12, top: 8, bottom: 8 }
                        flow: Down

                        log_content = <Markdown> {
                            width: Fill, height: Fit
                            font_size: 10.0
                            font_color: (GRAY_600)
                            paragraph_spacing: 4

                            draw_normal: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_600), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                            draw_bold: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_600), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                            draw_fixed: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 9.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((GRAY_600), (SLATE_400), self.dark_mode);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
