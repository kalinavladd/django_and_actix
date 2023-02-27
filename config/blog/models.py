from django.contrib.auth import get_user_model
from django.db import models

# Create your models here.


class Post(models.Model):
    title = models.CharField(max_length=120)
    text = models.CharField(max_length=250)
    author = models.ForeignKey(get_user_model(), on_delete=models.CASCADE)
