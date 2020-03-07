<script>
	import Weather from './Weather.svelte'
	import populations from './populations'

	function findCountry(population) {
		if (population == "") {
			return ""
		}

		population = +population

		return populations
			.map(country => {
				return {
					distance: Math.abs(population - country["2018"]),
					countryName: country["Country Name"]
				}
			})
			.sort((a, b) => a.distance - b.distance)
			[0].countryName
	}

	let typedValue = ""
	$: chosenCountry = findCountry(typedValue)



	async function getWeather(countryName) {
		if (countryName == "") {
			return Promise.resolve(null)
		}

		const apiKey = process.env.apiKey
		const apiUrl = `http://api.weatherstack.com/current?access_key=${apiKey}&query=${countryName}`

		const response = await fetch(apiUrl)
		const json = await response.json()

		return json.current
	}

	/*let weather = null
	async function fetchWeather() {
		weather = null
		weather = await getWeather(chosenCountry)
	}*/

	$: promise = getWeather(chosenCountry)

	//$: weather = getWeather(chosenCountry)
</script>



<main>
	<h1>Country Picker!</h1>

	<label>
		Select country by population count
		<input type="text" bind:value={typedValue} />
	</label>

	{#if chosenCountry != ""}
		<p>You have chosen {chosenCountry}</p>
	{/if}

	<!--<button on:click={fetchWeather}>Hämta vaedret ffs</button>-->

	{#await promise}
		<p>...waiting</p>
	{:then weather}
		{#if weather != null}
			<Weather weather={weather} />
		{/if}
	{:catch error}
		<p style="color: red">{error.message}</p>
	{/await}
</main>
