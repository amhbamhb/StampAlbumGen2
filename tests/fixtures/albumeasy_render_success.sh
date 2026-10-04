#!/bin/sh
printf '%s\n' '%PDF-test' > "${2%.txt}.pdf"
printf '%s\n' 'Album generated successfully.'
exit 0
