const initFastlyRecs = () => {
  const INTOBS_CONFIG = {
    root: null, // Use the viewport as the root
    rootMargin: '0px',
    threshold: 0.1 // Trigger when 10% of the target is visible
  }
  const MAX_RECS = 100

  let containerEl = null
  let recsGenerated = 0
  const initialTarget = document.querySelector('.artwork-facets')

  const makeContainerEl = () => {
    const wrapEl = document.createElement('div')
    wrapEl.classList.add('fastly-recs')
    wrapEl.classList.add('section__inset')
    wrapEl.innerHTML = `
      <div class='fastly-heading'>
        <h3>✨ For you: other artworks matching your interests</h3>
        <span class='fastly-credit'>Powered by <a href='https://www.fastly.com'><img src="/fastly/logo" /></a></span>
      </div>
    `
    containerEl = document.createElement('div')
    containerEl.classList.add('fastly-items')
    wrapEl.appendChild(containerEl)
    initialTarget.insertAdjacentElement('afterend', wrapEl)
  }

  const getRecommendations = async offset => {
    try {
      const response = await fetch(`/fastly/recommend?offset=${offset}`)
      const data = await response.json()
      return data
    } catch (error) {
      console.error('Error fetching data:', error)
      return {}
    }
  }

  const buildObjectPromo = ({
    objectURL,
    imageURL,
    title,
    artistByline,
    objectDate
  }) => {
    const newElement = document.createElement('div')
    newElement.classList.add('related-artwork')
    newElement.innerHTML = `
      <figure class="related-artwork-image__wrapper card__standard-image">
        <a class="related-artwork-image__link gtm__relatedartwork gtm__relatedartwork--has-image" href="${objectURL}" tabindex="0">
          <img class="related-artwork-image" src="${imageURL}" alt="${title}" loading="lazy" />
        </a>
      </figure>
      <h3 class="related-artwork-title card__title">
        <a class="gtm__relatedartwork gtm__relatedartwork--has-image" href="${objectURL}" tabindex="0">${title}</a>
      </h3>
      <div class="related-artwork-meta">
        <div class="related-artwork-artist">${artistByline}</div>
        <div class="related-artwork-date">${objectDate}</div>
      </div>
    `
    return newElement
  }

  // On intersect, fetch data, append, and move the observer to the new content
  const handleIntersect = (entries, obs) => {
    entries
      .filter(e => e.isIntersecting)
      .forEach(async entry => {
        obs.unobserve(entry.target)
        const { objects, recommenderTime } = await getRecommendations(
          recsGenerated
        )
        if (!objects?.length) return
        if (!containerEl) makeContainerEl()
        const newEls = objects.map(item => buildObjectPromo(item))
        newEls.forEach(el => containerEl.appendChild(el))
        recsGenerated += newEls.length
        if (recsGenerated < MAX_RECS) obs.observe(newEls[0])
        console.info(
          `✨ Recommendations generated in ${recommenderTime.toFixed(2)}ms ✨`
        )
      })
  }

  if (initialTarget) {
    const observer = new IntersectionObserver(handleIntersect, INTOBS_CONFIG)
    observer.observe(initialTarget)
  }
}

if (document.readyState === 'complete' || document.readyState === 'loaded') {
  initFastlyRecs()
} else {
  document.addEventListener('DOMContentLoaded', initFastlyRecs)
}
