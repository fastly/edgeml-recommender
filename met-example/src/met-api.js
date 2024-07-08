import { CacheOverride } from 'fastly:cache-override'


export const getObjectDetail = async objID => {
  const backend = 'met_api'

  try {
    const resp = await fetch(
      `https://collectionapi.metmuseum.org/public/collection/v1/objects/${objID}`,
      {
        backend,
        cacheOverride: new CacheOverride('override', {
          ttl: 86400,
          swr: 3600
        })
      }
    )
    if (!resp.ok) {
      throw new Error(`${resp.status} ${resp.statusText}`)
    }
    const rawData = await resp.json()
    return {
      objectURL: rawData.objectURL.replace(/^https?:\/\/.*?\//, '/'),
      imageURL:
        rawData.primaryImageSmall ||
        '/Rodan/dist/svg/no-image-image-related.svg',
      title: rawData.title,
      artistByline:
        rawData.artistDisplayName +
        (rawData.artistDisplayBio ? ' ' + rawData.artistDisplayBio : ''),
      objectDate: rawData.objectDate
    }
  } catch (err) {
    console.error(`[${backend}] fetch failed`, err)
    throw err
  }
}
