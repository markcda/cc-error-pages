use cc_ui_kit::{prelude::*, router::get_path};
use leptos_meta::*;
use lucide_leptos::MoveLeft;
use thaw::*;

leptos_i18n::load_locales!();
use crate::i18n::*;

fn main() {
  console_error_panic_hook::set_once();
  #[cfg(debug_assertions)]
  console_log::init_with_level(log::Level::Debug).unwrap();
  #[cfg(not(debug_assertions))]
  console_log::init_with_level(log::Level::Info).unwrap();
  leptos::mount::mount_to_body(|| {
    view! {
      <I18nContextProvider>
        <App />
      </I18nContextProvider>
    }
  })
}

#[component]
fn App() -> impl IntoView {
  provide_meta_context();
  let i18n = use_i18n();

  view! {
    <Title
      text=move || t_string!(i18n, title)
      formatter=move |text| format!("{text} - CC Services")
    />
    <main>
      {move || match get_path().unwrap().as_str() {
        "/400" => {
          view! {
            <ErrorPage err_num="400".to_string() err_msg=t_string!(i18n, bad_request).to_string() />
          }
            .into_any()
        }
        "/401" => {
          view! {
            <ErrorPage
              err_num="401".to_string()
              err_msg=t_string!(i18n, unauthorized).to_string()
            />
          }
            .into_any()
        }
        "/403" => {
          view! {
            <ErrorPage err_num="403".to_string() err_msg=t_string!(i18n, forbidden).to_string() />
          }
            .into_any()
        }
        "/" | "/404" => {
          view! {
            <ErrorPage err_num="404".to_string() err_msg=t_string!(i18n, not_found).to_string() />
          }
            .into_any()
        }
        "/405" => {
          view! {
            <ErrorPage err_num="405".to_string() err_msg=t_string!(i18n, not_allowed).to_string() />
          }
            .into_any()
        }
        "/423" => {
          view! {
            <ErrorPage err_num="423".to_string() err_msg=t_string!(i18n, locked).to_string() />
          }
            .into_any()
        }
        "/500" => {
          view! {
            <ErrorPage err_num="500".to_string() err_msg=t_string!(i18n, internal).to_string() />
          }
            .into_any()
        }
        _ => {
          view! {
            <ErrorPage err_num="???".to_string() err_msg=t_string!(i18n, specific).to_string() />
          }
            .into_any()
        }
      }}
    </main>
  }
}

#[component]
fn ErrorPage(err_num: String, err_msg: String) -> impl IntoView {
  let theme = RwSignal::new({
    let mut theme = Theme::light();
    theme.color.color_brand_background = "#17171a".to_string();
    theme.color.color_brand_background_hover = "#2c2c32".to_string();
    theme.color.color_brand_background_pressed = "#2c2c32".to_string();
    theme
  });

  let i18n = use_i18n();

  view! {
    <ConfigProvider theme>
      <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
        <h1 style="font-size: 72pt;" class="mb-10 text-gray-800 font-bold">
          {err_num}
        </h1>
        <p class="mb-4 text-xl text-gray-600">{err_msg}</p>
        <GoBack />
      </div>
    </ConfigProvider>
  }
}

fn get_referrer() -> String {
  web_sys::window().unwrap().document().unwrap().referrer()
}

fn redirect(uri: String) {
  web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .location()
    .unwrap()
    .set_href(uri.as_str())
    .unwrap()
}

#[component]
fn GoBack() -> impl IntoView {
  let i18n = use_i18n();
  let ref_is_empty = get_referrer().is_empty();
  let go_back = move || redirect(get_referrer());

  #[cfg(not(debug_assertions))]
  view! {
    <Show when=move || { !ref_is_empty }>
      <Button appearance=ButtonAppearance::Primary on_click=move |_| go_back()>
        <MoveLeft color="white" size=24 stroke_width=2 />
        <p class="ml-2">{move || t_string!(i18n, go_back)}</p>
      </Button>
    </Show>
  }
  #[cfg(debug_assertions)]
  view! {
    <Show when=move || { !ref_is_empty } fallback=move || view! { <a href="/404">"Go to 404"</a> }>
      <Button appearance=ButtonAppearance::Primary on_click=move |_| go_back()>
        <MoveLeft color="white" size=24 stroke_width=2 />
        <p class="ml-2">{move || t_string!(i18n, go_back)}</p>
      </Button>
    </Show>
  }
}
