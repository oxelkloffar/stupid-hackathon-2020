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

	$: promise = getWeather(chosenCountry)
</script>


<main>
	<h1>Intuitivest Weather App!</h1>

	<label>
		Select country by population count
		<input type="text" bind:value={typedValue} />
	</label>

	{#if chosenCountry != ""}
		<h2>{chosenCountry}</h2>
	{/if}

	{#await promise}
		<p>Loading...</p>
	{:then weather}
		{#if weather != null}
			<Weather weather={weather} />
		{/if}
	{:catch error}
		<p style="color: red">{error.message}</p>
	{/await}
</main>


<style>
	h2, p {
		margin-left: 25px;
	}
</style>
