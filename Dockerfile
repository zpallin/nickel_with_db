# nwdb_mongo image
#   zpallin
#   2016
#
# right now there is no difference from the typical mongo container, but I will update this
# to include base scaffolding at some point

FROM mongo:3.3
EXPOSE 27017
CMD ["mongod"]	
