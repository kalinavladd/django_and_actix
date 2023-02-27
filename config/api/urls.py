from django.urls import path, include
from rest_framework import routers
from rest_framework_simplejwt.views import (
    TokenObtainPairView,
    TokenRefreshView,
)

from api.users.viewsets import PublicUserViewSet

from api.blog.viewsets import ProjectApiViewSet

router = routers.DefaultRouter()

router.register('', PublicUserViewSet, basename='users')
router.register('blog', ProjectApiViewSet, basename='blog')


urlpatterns = [
    path('', include(router.urls)),
    path('token/', TokenObtainPairView.as_view(), name='token_obtain_pair'),
    path('token/refresh/', TokenRefreshView.as_view(), name='token_refresh'),
]

app_name = "api"