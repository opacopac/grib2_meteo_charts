extern crate core;

use clap::Parser;
use meteo_grib2_renderer::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer::IconD2ForecastRenderer;
use meteo_grib2_renderer::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer::IconCh1ForecastRenderer;
use meteo_grib2_renderer::prod_di_container::ProdDiContainer;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// name of the model to render (e.g. icon-ch1, icon-d2)
    #[arg(short, long)]
    model: String,

    /// name of the variables to render (e.g. wind_10m, temp_2m, cloud_precip) or empty for all
    #[arg(short, long, num_args = 1..)]
    variables: Vec<String>,

    /// number of the steps to render (e.g. 2, 3, 4...) or empty for all
    #[arg(short, long, num_args = 1..)]
    steps: Vec<usize>,
}


fn main() {
    env_logger::init();
    let _ = ProdDiContainer::new();

    let args = Args::parse();

    match args.model.as_str() {
        "icon-ch1" => {
            let _ = IconCh1ForecastRenderer::render_latest_forecasts(&args.variables, &args.steps)
                .or_else(|e| {
                    println!("error while rendering icon-ch1 forecast: {}", e);
                    Err(e)
                });
        }
        "icon-d2" => {
            let _ = IconD2ForecastRenderer::render_latest_forecasts(&args.variables, &args.steps)
                .or_else(|e| {
                    println!("error while rendering icon-d2 forecast: {}", e);
                    Err(e)
                });
        }
        _ => {
            println!("unknown model: {}", args.model);
        }
    }
}


#[cfg(test)]
mod tests {
    use meteo_grib2_renderer::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer::IconCh1ForecastRenderer;

    #[test]
    #[ignore = "only for manual testing"]
    fn it_renders_the_icon_ch1_charts() {
        // given
        let variable_filter = vec!["vertical_cloud".to_string()]; // vec!["temp_2m".to_string()];
        let step_filter = vec![2];

        // when
        let result = IconCh1ForecastRenderer::render_latest_forecasts(&variable_filter, &step_filter);

        // then
        assert!(result.is_ok());
    }
}
