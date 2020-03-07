<script>
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
</main>
