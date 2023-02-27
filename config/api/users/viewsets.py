from django.middleware.csrf import get_token
from django.contrib.auth import (
    authenticate,
    login as auth_login,
    logout as auth_logout)

from rest_framework import viewsets, status
from rest_framework.response import Response
from rest_framework.permissions import IsAuthenticated, AllowAny
from rest_framework.decorators import action
from rest_framework.serializers import ValidationError

from django.contrib.auth import get_user_model
from .serializers import LoginSerializer

User = get_user_model()


class PublicUserViewSet(viewsets.GenericViewSet):
    def get_queryset(self):
        return User.objects.all()

    @action(
        ['post'], detail=False, permission_classes=[AllowAny],
        serializer_class=LoginSerializer)
    def login(self, request):
        serializer = LoginSerializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        user = authenticate(**serializer.validated_data)
        if not user:
            raise ValidationError(
                "Login or password are incorrect")
        auth_login(request, user)
        return Response({"message": "success"})

    @action(
        ['get'], detail=False, permission_classes=[IsAuthenticated])
    def logout(self, request):
        auth_logout(request)
        return Response({"message": "success"})