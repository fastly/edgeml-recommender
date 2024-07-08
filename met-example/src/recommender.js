const RECOMMENDER_URL = 'https://wholly-proven-reindeer.edgecompute.app'

export const getSuggestions = async (viewedIDs, offset = 0, recs = 10) => {
  const backend = 'recommender'

  try {
    const apiUrl = new URL(RECOMMENDER_URL)
    apiUrl.searchParams.append('offset', offset)
    apiUrl.searchParams.append('recs', recs)
    apiUrl.searchParams.append('ids', viewedIDs)
    const resp = await fetch(apiUrl, { backend })
    if (!resp.ok) {
      throw new Error(`${resp.status} ${resp.statusText}`)
    }
    const results = await resp.json()
    return results
  } catch (e) {
    console.error(`[${backend}] fetch failed`, err)
    return []
  }
}
