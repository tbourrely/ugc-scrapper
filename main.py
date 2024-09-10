#!/usr/bin/env python3

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

def get_movie_details(movie_element, date):
    movie = {}
    title = movie_element.select('.block--title > a')[0]
    movie['title'] = title.text.strip()
    screenings = movie_element.select('.screening-start')
    movie['screenings'] = [str(date) + " " + s.text.strip() for s in screenings] # only hours display in html
    return movie

def with_screenings(movies):
    return list(filter(lambda x: len(x.get('screenings')) > 0, movies))

def has_screening_for_theater(screenings, theater):
    for screening in screenings:
        if screening.get('theater') == theater:
            return True
    return False

dates = []
for i in range(1, 7):
    dates.append(datetime.date.today() + datetime.timedelta(days=i))

grouped_movies = {} 

for key, value in THEATERS.items():
    theater_movies = {}
    for date in dates:
        movies_page = get_theater_movies(value, date)
        movies = with_screenings([get_movie_details(movie_page, date) for movie_page in movies_page])

        for movie in movies:
            if movie.get('title') in theater_movies:
                theater_movies[movie.get('title')].extend(movie.get('screenings'))
            else:
                theater_movies[movie.get('title')] = movie.get('screenings')

    # flatten movies across all theaters
    # movie 
    # > theater
    # > > screenings[]
    for title, screenings in theater_movies.items():
        if title not in grouped_movies:
            grouped_movies[title] = {}
        grouped_movies[title][key] = screenings

__import__('pprint').pprint(grouped_movies)
