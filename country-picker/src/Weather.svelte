<script>
	export let weather

	function ucwords(str) {
    return (str + '').replace(/^([a-z])|\s+([a-z])/g, function ($1) {
        return $1.toUpperCase()
    })
	}

	function weatherDescriptionToClass(weather) {
		const description = ucwords(weather.weather_descriptions[0])
		let iconClass = 'full_clouds'
		let weatherDescription = description
		
		switch (description) {
			case 'Partly Cloudy':
			iconClass = 'partly_cloudy';
			break;
			
			case 'Haze':
			case 'Overcast':
			iconClass = 'full_clouds';
			break;
			
			case 'Clear':
			iconClass = 'night';
			break;
			
			case 'Patchy Light Drizzle':
			iconClass = 'sun_rain_clouds';
			weatherDescription = 'Light Drizzle';
			break;
			
			case 'Sunny':
			iconClass = 'full_sun';
			break;
			
			case 'Patchy Rain Possible':
			iconClass = 'cloud_slight_rain';
			weatherDescription = 'Patchy Rain';
			break;
			
			case 'Light Rain':
			case 'Light Rain, Mist':
			iconClass = 'cloud_slight_rain';
			break;
			
			case 'Moderate Or Heavy Rain Shower':
			iconClass = 'rainy';
			weatherDescription = 'Heavy Rain';
			break;
			
			case 'Thunder':
			iconClass = 'thunder';
			break;
			
			default: 
			iconClass = 'full_clouds';
			break;	
		
			// some may be missing 
		}
		
		return [iconClass, weatherDescription];
	}

	$: parsedWeather = weatherDescriptionToClass(weather)
</script>


<section>
	<i class={parsedWeather[0]}></i>
	<p>{parsedWeather[1]}</p>
</section>


<style>
section {
	position: relative;
	width: 275px;
	height: 137px;
	margin-top: -30px;
}

p {
	position: absolute;
	font: 200%;
	width: 140px;
	text-align: center;
	top: 120px;
	left: 22px;
}

i {
	background: url(/images/weather_icon_partly_cloudy.svg);
	background-size: cover;
	background-repeat: no-repeat;
	width: 140px;
	height: 137px;
	left: 22px;
	position: absolute;
}
i.partly_cloudy {
	background: url(/images/weather_icon_partly_cloudy.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 19px !important;
}
i.full_clouds {
	background: url(/images/weather_icon_full_clouds.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 19px !important;
}
i.night {
	background: url(/images/weather_icon_night.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 16px !important;
}
i.sun_rain_clouds {
	background: url(/images/weather_icon_sun_rain_clouds.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 15px !important;
}
i.full_sun {
	background: url(/images/weather_icon_full_sun.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 17px !important;
}
i.rainy {
	background: url(/images/weather_icon_rainy.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 13px !important;
}
i.cloud_slight_rain {
	background: url(/images/weather_icon_cloud_slight_rain.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 13px !important;
}
i.thunder {
	background: url(/images/weather_icon_thunder.svg);
	background-size: cover;
	background-repeat: no-repeat;
	top: 0px;
}
</style>