# Met Museum Proxy

A proxy for [The Metropolitan Museum of Art website](https://www.metmuseum.org). It queries a [recommendation engine](../recommender/) to give on-page, instant & personalized recommendations based on the visitor's browsing history. All on [Fastly Compute](https://www.fastly.com/documentation/guides/compute/).

![Instant recommendations in action](https://github.com/doramatadora/edgeml-recommender/assets/12828487/1ff6db7a-a57b-4987-a1a6-e14f7112b9d6)

This code was lightly modified from [@triblondon](https://github.com/triblondon)'s original Met proxy at [`fastly/compute-recommender-met-demo`](https://github.com/fastly/compute-recommender-met-demo). Check out [the excellent explanation video](https://www.youtube.com/watch?v=1oheoNras9Q) on Fastly Developers Live.

## See it in action

Go to [https://edgeml-recommender.edgecompute.app/art/collection/search/1](https://edgeml-recommender.edgecompute.app/art/collection/search/1) and start browsing around. 

As you browse, your personalised recommendations will be displayed on-page, under the `✨ For you: other artworks matching your interests` heading.

Open the developer console to see the recommendation engine backend response time:

```
✨ Recommendations generated in 46.39ms ✨`
```

That's how quickly it selected artwork recommendations from the Met Museum's half-a-million-strong collection based on your recent browsing history, using ML inference on Fastly Compute. For context, the average time it takes for a human to blink is around 400 milliseconds!

> Note: The difference between the console-logged time and the network request time to the `/recommend` endpoint accounts for requests to the [Met Museum's Collection API](https://collectionapi.metmuseum.org/), to load object descriptions and images.
>
> You can make requests directly to the recommendation engine using comma-separated object IDs from the Met's collection: 
> ```
> https://edgeml-recommender-engine.edgecompute.app/?ids=1,2,3
> ```
