# aber_webring
A webring for the websites of students of aberystwyth

## What is a webring?
A [webring](https://en.wikipedia.org/wiki/Webring) is a collection of websites in a circular structure, each one has left and right buttons leading to the next websites in the ring, its essentially just a fun easter-egg to hide on your site.

## How does this work?
At the moment this is a work-in-progress project but eventually itll be a lightweight server-side application. Members of the webring will add arrows that link to the server with data on what website they are and what direction to go. The server will then redirect the user to the next site in the ring. The hope being that this would be more resiliant than a basic hardcoded webring. This way we can add new members and remove dead websites automatically without anyone having to change anything!

### Adding your link
To join the webring make sure your site is following the below criteria then just create a pull request adding yourself to the pages.txt file and add the webring buttons somewhere on your site. Heres a really basic implementation if you're wanting a simple one:
```html
<div class="webring">
    <h2> Webring </h2>
    <div class="webringContent">
        <a href="users.aber.ac.uk/evh14"><-</a>
        <img src="webringlogo.svg" style="margin: 0; width: 19px;">
        <a href="users.aber.ac.uk/evh14">-></a>
    </div>
</div>

```
```css
.webring{
    position: fixed;
    bottom: 0;
    left: 0;

    margin: 5px;
}

.webring h2{
    font-size: 1rem;
    margin: 0;
}

.webringContent{
    display: flex;
    gap: 0;
}
```

You can find the svg icon there in this repo, its initially taken from [The XXIIVV Webring](https://github.com/XXIIVV/webring) using their MIT license. Go check out that project, every page on there is incredibly interesting!

## Criteria
If you're wanting to add yourself to this, please make sure your page doesnt contain any hateful content or anything inappropriate. 

This is up to your discretion but just ask yourself whether other people would be okay linking their website (possibly a portfolio or recruiter facing site) to yours, if theres a chance they wouldnt then best avoid it.

Make sure your website represents you! This is meant for personal sites, not projects or adverts.

