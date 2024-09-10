import datetime
import requests
from bs4 import BeautifulSoup

THEATERS = {
    'confluence': 36,
    'astoria': 33,
    'part-dieu': 58,
    'cite internationale': 33,
}

THEATER_URL = 'https://www.ugc.fr/showingsCinemaAjaxAction!getShowingsForCinemaPage.action?cinemaId={:d}&date={}'

def get_theater_movies(theater_id, date):
    url = THEATER_URL.format(theater_id, date)
    movies_page = requests.get(
        url,
        headers={
            "Accept-Language": "fr-FR",
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8",
            "Host": "www.ugc.fr",
            "User-Agent": "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"
        }
    )
    soup = BeautifulSoup(movies_page.text, "html.parser")
    movies = soup.select('.component--cinema-list-item')
    return movies

def get_movie_details(movie_element):
    movie = {}
    title = movie_element.select('.block--title > a')[0]
    movie['title'] = title.text.strip()

    screenings = movie_element.select('.screening-start')
    movie['screenings'] = [s.text.strip() for s in screenings]

    return movie

def with_screenings(movies):
    return list(filter(lambda x: len(x.get('screenings')) > 0, movies))

tomorrow = datetime.date.today() + datetime.timedelta(days=1)

grouped_movies = {} 
for key, value in THEATERS.items():
    movies_page = get_theater_movies(value, tomorrow)
    movies = with_screenings([get_movie_details(movie_page) for movie_page in movies_page])

    # group movies by title
    # and add screenings in a list for each theater
    for movie in movies:
        if movie.get('title') in grouped_movies:
            grouped_movies[movie.get('title')].append({ 'theater': key, 'screenings': movie.get('screenings') })
        else:
            grouped_movies[movie.get('title')] = [{ 'theater': key, 'screenings': movie.get('screenings') }]

__import__('pprint').pprint(grouped_movies)
