from django.http import HttpResponse

def health_check_admin(request):
    return HttpResponse('OK')
