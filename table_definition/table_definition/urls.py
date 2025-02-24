"""
URL configuration for table_definition project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path
from django.conf import settings
from django.conf.urls.static import static
# from admin_server.views import h

from admin_server.views import health_check_admin


urlpatterns = [
    # path('admin/', admin.site.urls),
    path(f'{settings.CUSTOM_ADMIN_ROOT}health-check', health_check_admin),
    path(settings.CUSTOM_ADMIN_ROOT, admin.site.urls),
]

urlpatterns += static(f"{settings.STATIC_URL}", document_root=settings.STATIC_ROOT)
